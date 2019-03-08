//! This module contains an implementation of a threshold clock.
//!
//! The concept of threshold-union is explained in detail in [this blog post](https://vitorenes.org/post/2018/11/threshold-union/).
//!
//! # Examples
//! ```
//! use threshold::{vclock, *};
//!
//! let vclock_0 = vclock::from_seqs(vec![10, 5, 5]);
//! let vclock_1 = vclock::from_seqs(vec![8, 10, 6]);
//!
//! let mut tclock = TClock::new();
//! tclock.add(vclock_0);
//! tclock.add(vclock_1);
//!
//! let vclock_t1 = vclock::from_seqs(vec![10, 10, 6]);
//! let vclock_t2 = vclock::from_seqs(vec![8, 5, 5]);
//!
//! assert_eq!(tclock.threshold_union(1), vclock_t1);
//! assert_eq!(tclock.threshold_union(2), vclock_t2);
//!
//! assert_eq!(tclock.threshold_union(1), tclock.union());
//! assert_eq!(tclock.threshold_union(2), tclock.intersection());
//! ```

use crate::*;
use std::collections::HashMap;

pub struct TClock<T: Actor> {
    /// Number of clocks added to the `TClock`
    clock_count: u64,
    /// A `MultiSet` per `Actor`
    occurrences: HashMap<T, MultiSet<u64>>,
}

impl<T: Actor> TClock<T> {
    /// Returns a new `TClock` instance.
    pub fn new() -> Self {
        TClock {
            clock_count: 0,
            occurrences: HashMap::new(),
        }
    }

    /// Adds a `VClock` to the `TClock`.
    ///
    /// # Examples
    /// ```
    /// use threshold::{vclock, *};
    ///
    /// let mut tset = TClock::new();
    /// assert_eq!(tset.clock_count(), 0);
    ///
    /// let vclock = vclock::from_seqs(1..10);
    /// tset.add(vclock);
    /// assert_eq!(tset.clock_count(), 1);
    /// ```
    pub fn add(&mut self, clock: VClock<T>) {
        self.clock_count += 1;

        for (actor, seq) in clock {
            self.add_entry(actor, seq);
        }
    }

    /// Adds a single clock entry to the `TClock`.
    fn add_entry(&mut self, actor: T, seq: u64) {
        match self.occurrences.get_mut(&actor) {
            Some(mset) => {
                mset.add_elem(seq);
            }
            None => {
                self.occurrences.insert(actor, MultiSet::singleton(seq));
            }
        }
    }

    /// Returns the number of clocks added to the `TClock`.
    pub fn clock_count(&self) -> u64 {
        self.clock_count
    }

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
    /// let vclock_0 = vclock::from_seqs(vec![10, 5, 5]);
    /// let vclock_1 = vclock::from_seqs(vec![8, 10, 6]);
    /// let vclock_2 = vclock::from_seqs(vec![9, 8, 7]);
    ///
    /// let mut tclock = TClock::new();
    /// tclock.add(vclock_0);
    /// tclock.add(vclock_1);
    /// tclock.add(vclock_2);
    ///
    /// let vclock_t1 = vclock::from_seqs(vec![10, 10, 7]);
    /// let vclock_t2 = vclock::from_seqs(vec![9, 8, 6]);
    /// let vclock_t3 = vclock::from_seqs(vec![8, 5, 5]);
    ///
    /// assert_eq!(tclock.threshold_union(1), vclock_t1);
    /// assert_eq!(tclock.threshold_union(2), vclock_t2);
    /// assert_eq!(tclock.threshold_union(3), vclock_t3);
    /// ```
    pub fn threshold_union(&self, threshold: u64) -> VClock<T> {
        let mut map = HashMap::new();

        for (actor, tset) in self.occurrences.iter() {
            let mut positives = 0;

            // get the highest sequence that passes the threshold
            let seq = tset
                .iter()
                .rev()
                .skip_while(|(_, &count)| {
                    positives += count;
                    positives < threshold
                })
                .next()
                .map_or(0, |(&seq, _)| seq);

            // insert it in the map
            map.insert(actor.clone(), seq);
        }

        VClock::from_map(map)
    }

    /// Computes the union of all clocks added to the `TClock`.
    ///
    /// # Examples
    /// ```
    /// use threshold::{vclock, *};
    ///
    /// let vclock_0 = vclock::from_seqs(vec![10, 5, 5]);
    /// let vclock_1 = vclock::from_seqs(vec![8, 10, 6]);
    ///
    /// let mut tclock = TClock::new();
    /// tclock.add(vclock_0);
    /// tclock.add(vclock_1);
    /// assert_eq!(tclock.union(), vclock::from_seqs(vec![10, 10, 6]));
    /// ```
    pub fn union(&self) -> VClock<T> {
        self.threshold_union(1)
    }

    /// Computes the intersection of all clocks added to the `TClock`.
    ///
    /// # Examples
    /// ```
    /// use threshold::{vclock, *};
    ///
    /// let vclock_0 = vclock::from_seqs(vec![10, 5, 5]);
    /// let vclock_1 = vclock::from_seqs(vec![8, 10, 6]);
    ///
    /// let mut tclock = TClock::new();
    /// tclock.add(vclock_0);
    /// tclock.add(vclock_1);
    /// assert_eq!(tclock.intersection(), vclock::from_seqs(vec![8, 5, 5]));
    /// ```
    pub fn intersection(&self) -> VClock<T> {
        self.threshold_union(self.clock_count)
    }
}
