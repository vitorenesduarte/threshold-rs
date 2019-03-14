//! This module contains an implementation of a threshold clock.
//!
//! The concept of threshold-union is explained in detail in [this blog post](https://vitorenes.org/post/2018/11/threshold-union/).
//!
//! # Examples
//! ```
//! use threshold::{vclock, *};
//!
//! let vclock_0 = vclock::vclock_from_seqs(vec![10, 5, 5]);
//! let vclock_1 = vclock::vclock_from_seqs(vec![8, 10, 6]);
//!
//! let mut tclock = TClock::new();
//! tclock.add(vclock_0);
//! tclock.add(vclock_1);
//!
//! let vclock_t1 = vclock::vclock_from_seqs(vec![10, 10, 6]);
//! let vclock_t2 = vclock::vclock_from_seqs(vec![8, 5, 5]);
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
    /// use threshold::{vclock, *};
    ///
    /// let mut tset = TClock::new();
    ///
    /// let vclock = vclock::vclock_from_seqs(1..10);
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
    /// of all clocks added to the `TClock`.
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
    /// use threshold::{vclock, *};
    /// let vclock_0 = vclock::vclock_from_seqs(vec![10, 5, 5]);
    /// let vclock_1 = vclock::vclock_from_seqs(vec![8, 10, 6]);
    /// let vclock_2 = vclock::vclock_from_seqs(vec![9, 8, 7]);
    ///
    /// let mut tclock = TClock::new();
    /// tclock.add(vclock_0);
    /// tclock.add(vclock_1);
    /// tclock.add(vclock_2);
    ///
    /// let vclock_t1 = vclock::vclock_from_seqs(vec![10, 10, 7]);
    /// let vclock_t2 = vclock::vclock_from_seqs(vec![9, 8, 6]);
    /// let vclock_t3 = vclock::vclock_from_seqs(vec![8, 5, 5]);
    ///
    /// assert_eq!(tclock.threshold_union(1), vclock_t1);
    /// assert_eq!(tclock.threshold_union(2), vclock_t2);
    /// assert_eq!(tclock.threshold_union(3), vclock_t3);
    /// ```
    pub fn threshold_union(&self, threshold: u64) -> VClock<A> {
        let iter = self.occurrences.iter().map(|(actor, tset)| {
            let mut total_count = 0;

            // get the highest sequence that passes the threshold
            let seq = tset
                .iter()
                .rev()
                .skip_while(|(_, &(count, _))| {
                    // `total_count` records the implicit number of
                    // observations: since we are iterating from the highest
                    // event to the lowest, and the observation of event X
                    // counts as an observation of event Y when X > Y, we
                    // can simply accumulate all observations in `total_count`
                    // and stop the `skip_while` once `total_count` passes the
                    // threshold
                    total_count += count;
                    total_count < threshold
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
