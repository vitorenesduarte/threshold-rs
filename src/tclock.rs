//! This module contains an implementation of a threshold clock.
//!
//! The concept of threshold-union is explained in detail in [this blog post](https://vitorenes.org/post/2018/11/threshold-union/).
//!
//! # Examples
//! ```
//! use threshold::{clock, *};
//!
//! let vclock_0 = clock::vclock_from_seqs(vec![10, 5, 5]);
//! let vclock_1 = clock::vclock_from_seqs(vec![8, 10, 6]);
//!
//! let mut tclock = TClock::new();
//! tclock.add(vclock_0);
//! tclock.add(vclock_1);
//!
//! let vclock_t1 = clock::vclock_from_seqs(vec![10, 10, 6]);
//! let vclock_t2 = clock::vclock_from_seqs(vec![8, 5, 5]);
//!
//! assert_eq!(tclock.threshold_union(1), vclock_t1);
//! assert_eq!(tclock.threshold_union(2), vclock_t2);
//! ```

use crate::*;
use std::collections::HashMap;
use std::marker::PhantomData;

type EventCount = (u64, u64);

pub struct TClock<A: Actor, E: EventSet> {
    /// A `MultiSet` per `Actor`
    occurrences: HashMap<A, MultiSet<u64, EventCount>>,
    phantom: PhantomData<E>,
}

impl<A: Actor, E: EventSet> TClock<A, E> {
    /// Returns a new `TClock` instance.
    pub fn new() -> Self {
        TClock {
            occurrences: HashMap::new(),
            phantom: PhantomData,
        }
    }

    /// Add a `Clock` to the `TClock`.
    ///
    /// # Examples
    /// ```
    /// use threshold::{clock, *};
    ///
    /// let mut tset = TClock::new();
    ///
    /// let vclock = clock::vclock_from_seqs(1..10);
    /// tset.add(vclock);
    /// ```
    pub fn add(&mut self, clock: Clock<A, E>) {
        for (actor, eset) in clock {
            self.add_entry(actor, eset);
        }
    }

    /// Adds a single clock entry to the `TClock`.
    fn add_entry(&mut self, actor: A, eset: E) {
        // compute event count
        let count = event_count(eset);

        match self.occurrences.get_mut(&actor) {
            Some(mset) => {
                // if we have other events from this actor
                // add new events to its multiset
                mset.add(count);
            }
            None => {
                // otherwise create a new multiset for this actor
                self.occurrences.insert(actor, MultiSet::from(count));
            }
        }
    }
}

impl<A: Actor> TClock<A, MaxSet> {
    /// Computes the [threshold-union](https://vitorenes.org/post/2018/11/threshold-union/)
    /// of all `VClock` added to the `TClock`.
    ///
    /// Assume multiset `X` is `{10: 1, 8: 2, 6: 3, 5: 1}`.
    /// This means that event `10` was seen once, event `8` twice, and so on.
    ///
    /// (Recall that for vector clocks, seeing event 10 means seeing all events
    /// from 1 to 10.)
    ///
    /// If, for example, we want the event that was seen at least 4 times (i.e.
    /// our threshold is 4), we should get event `6`.
    ///
    /// Assume `threshold(u64, X) -> Option<u64>` where the first argument is
    /// the threshold desired. Then:
    /// - `threshold(1, X) = Some(10)`
    /// - `threshold(2, X) = Some(8)`
    /// - `threshold(3, X) = Some(8)`
    /// - `threshold(4, X) = Some(6)`
    /// - `threshold(7, X) = Some(5)`
    /// - `threshold(8, X) = None`
    ///
    /// # Examples
    /// ```
    /// use threshold::{clock, *};
    /// let vclock_0 = clock::vclock_from_seqs(vec![10, 5, 5]);
    /// let vclock_1 = clock::vclock_from_seqs(vec![8, 10, 6]);
    /// let vclock_2 = clock::vclock_from_seqs(vec![9, 8, 7]);
    ///
    /// let mut tclock = TClock::new();
    /// tclock.add(vclock_0);
    /// tclock.add(vclock_1);
    /// tclock.add(vclock_2);
    ///
    /// let vclock_t1 = clock::vclock_from_seqs(vec![10, 10, 7]);
    /// let vclock_t2 = clock::vclock_from_seqs(vec![9, 8, 6]);
    /// let vclock_t3 = clock::vclock_from_seqs(vec![8, 5, 5]);
    ///
    /// assert_eq!(tclock.threshold_union(1), vclock_t1);
    /// assert_eq!(tclock.threshold_union(2), vclock_t2);
    /// assert_eq!(tclock.threshold_union(3), vclock_t3);
    /// ```
    pub fn threshold_union(&self, threshold: u64) -> VClock<A> {
        let iter = self.occurrences.iter().map(|(actor, tset)| {
            let mut total_pos = 0;

            // get the highest sequence that passes the threshold
            let seq = tset
                .iter()
                .rev()
                .skip_while(|(_, &(pos, _))| {
                    // `total_pos` records the implicit number of observations:
                    // since we are iterating from the highest event to the
                    // lowest, and the observation of event X counts as an
                    // observation of event Y when X > Y, we can simply
                    // accumulate all observations in `total_pos` and stop the
                    // `skip_while` once `total_pos` passes the threshold
                    total_pos += pos;
                    total_pos < threshold
                })
                .next()
                // if there is an event that passes the threshold, return it
                // otherwise, return `0`
                .map_or_else(
                    || MaxSet::new(),
                    |(&seq, _)| MaxSet::from_event(seq),
                );

            (actor.clone(), seq)
        });

        VClock::from(iter)
    }
}

impl<A: Actor> TClock<A, BelowExSet> {
    /// Computes the [threshold-union](https://vitorenes.org/post/2018/11/threshold-union/)
    /// of all `BEClock` added to the `TClock`.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut clock_a = BEClock::new();
    /// clock_a.add_dot(&Dot::new(&Musk::B, 5));
    /// clock_a.add_dot(&Dot::new(&Musk::B, 6));
    ///
    /// let mut clock_b = BEClock::new();
    /// clock_b.add_dot(&Dot::new(&Musk::B, 5));
    /// clock_b.add_dot(&Dot::new(&Musk::B, 7));
    ///
    /// let mut tclock = TClock::new();
    /// tclock.add(clock_a);
    /// tclock.add(clock_b);
    ///
    /// let mut expected = BEClock::new();
    /// expected.add_dot(&Dot::new(&Musk::B, 5));
    ///
    /// assert_eq!(tclock.threshold_union(2), expected);
    /// ```
    pub fn threshold_union(&self, threshold: u64) -> BEClock<A> {
        let iter = self.occurrences.iter().map(|(actor, tset)| {
            let mut total_pos = 0;

            // skip until some entry passes the threshold
            let iter = tset
                .iter()
                .rev()
                .skip_while(|(_, &(pos, _))| {
                    // `total_pos` records the implicit number of observations:
                    // since we are iterating from the highest event to the
                    // lowest, and the observation of event X counts as an
                    // observation of event Y when X > Y, we can simply
                    // accumulate all observations in `total_pos` and stop the
                    // `skip_while` once `total_pos` passes the threshold
                    total_pos += pos;
                    total_pos < threshold
                })
                // had to collect here so that the borrow of `total_pos` ends
                // TODO can we avoid this?
                .collect::<Vec<_>>();
            let mut iter = iter.iter().peekable();

            let highest = match iter.next() {
                None => Ok(0),
                Some((&seq, &(_, neg))) => {
                    // check if the highest seq that passes the positive
                    // threshold is valid, i.e. if it still passes the threshold
                    // after subtracting the negative votes
                    if total_pos - neg >= threshold {
                        // if yes, this is the highest sequence
                        Ok(seq)
                    } else {
                        // if not, the highest sequence may not have received
                        // any of vote, i.e. it is not in the structure
                        Err(seq)
                    }
                }
            }
            .unwrap_or_else(|seq| {
                // if the highest `seq` that passed the positive threshold is
                // not the highest sequence we are looking for, then any
                // sequence smaller than `seq` could be the highest sequence
                let mut candidate = seq - 1;
                loop {
                    match iter.peek() {
                        None => {
                            // if the structure is empty, then we've found the
                            // highest sequence
                            break candidate;
                        }
                        Some((&next_seq, &(pos, neg))) => {
                            if next_seq == candidate {
                                // if the `candidate` is in the structure
                                // advance the iterator (this is fine since this
                                // candidate will be never be an exception)
                                iter.next();

                                // accumulate more positives
                                total_pos += pos;

                                if total_pos - neg >= threshold {
                                    // if `candidate` passes the threshold, then
                                    // we've found the highest sequence
                                    break candidate;
                                } else {
                                    // otherwise, try another sequence
                                    candidate -= 1;
                                }
                            } else {
                                // if the `candidate` is not in the structure,
                                // then we've found the highest sequence
                                break candidate;
                            }
                        }
                    }
                }
            });

            // compute exceptions:
            // - if there are any exceptions, they are part of our structure
            let exs = iter.filter_map(|(&seq, &(pos, neg))| {
                // accumulate more positives
                total_pos += pos;

                // if `total_pos - neg < threshold`, we have found an exception
                // - the `neg > total_pos` is here just to prevent `total_pos -
                //   neg` to overflow
                if neg > total_pos || total_pos - neg < threshold {
                    Some(seq)
                } else {
                    None
                }
            });

            let below_exset = BelowExSet::from(highest, exs);
            (actor.clone(), below_exset)
        });

        BEClock::from(iter)
    }
}

fn event_count<E: EventSet>(
    eset: E,
) -> impl Iterator<Item = (u64, EventCount)> {
    // get events
    let (left, right) = eset.events();

    // compute left event count
    let left_count = std::iter::once(left).map(|x| (x, (1, 0)));

    // compute right events count
    let right_count = right.into_iter().map(|x| (x, (0, 1)));

    // chain both
    left_count.chain(right_count)
}

#[test]
fn regression_test() {
    // Clock { clock: {B: BelowExSet { max: 6, exs: {1, 2, 3, 4} }} }
    let mut clock_a = BEClock::new();
    clock_a.add_dot(&Dot::new(&Musk::B, 5));
    clock_a.add_dot(&Dot::new(&Musk::B, 6));

    // Clock { clock: {B: BelowExSet { max: 7, exs: {1, 2, 3, 4, 6} }} }
    let mut clock_b = BEClock::new();
    clock_b.add_dot(&Dot::new(&Musk::B, 5));
    clock_b.add_dot(&Dot::new(&Musk::B, 7));

    // add both clocks to the threshold clock
    let mut tclock = TClock::new();
    tclock.add(clock_a);
    tclock.add(clock_b);

    // compute the threshold union
    let clock = tclock.threshold_union(2);

    // create the expected clock
    let mut expected = BEClock::new();
    expected.add_dot(&Dot::new(&Musk::B, 5));

    assert_eq!(clock, expected);
}
