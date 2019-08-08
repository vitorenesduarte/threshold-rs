//! This module contains an implementation of a above-extra set.
//!
//! # Examples
//! ```
//! use threshold::*;
//!
//! let mut above_exset = AboveExSet::new();
//! assert_eq!(above_exset.next_event(), 1);
//! assert!(above_exset.is_event(&1));
//! assert!(!above_exset.is_event(&2));
//!
//! let other = AboveExSet::from_event(3);
//! assert!(!other.is_event(&1));
//! assert!(!other.is_event(&2));
//! assert!(other.is_event(&3));
//!
//! above_exset.join(&other);
//! assert!(above_exset.is_event(&1));
//! assert!(!above_exset.is_event(&2));
//! assert!(above_exset.is_event(&3));
//! ```

use crate::EventSet;
use std::collections::BTreeSet;
use std::iter::FromIterator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AboveExSet {
    // Highest contiguous event seen
    max: u64,
    // Set of extra events above the highest (sorted ASC)
    // see https://doc.rust-lang.org/std/collections/struct.BTreeSet.html#min-heap
    exs: BTreeSet<u64>,
}

impl EventSet for AboveExSet {
    /// Returns a new `AboveExSet` instance.
    fn new() -> Self {
        AboveExSet {
            max: 0,
            exs: BTreeSet::new(),
        }
    }

    /// Creates a new instance from `event`.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let above_exset = AboveExSet::from_event(3);
    /// assert!(!above_exset.is_event(&1));
    /// assert!(!above_exset.is_event(&2));
    /// assert!(above_exset.is_event(&3));
    /// assert!(!above_exset.is_event(&4));
    /// ```
    fn from_event(event: u64) -> Self {
        match event {
            1 => AboveExSet {
                max: 1,
                exs: BTreeSet::new(),
            },
            _ => AboveExSet {
                max: 0,
                exs: [event].iter().cloned().collect(),
            },
        }
    }

    /// Generates the next event.
    /// There should be no extras when calling this.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut above_exset = AboveExSet::new();
    /// assert_eq!(above_exset.next_event(), 1);
    /// assert_eq!(above_exset.next_event(), 2);
    /// ```
    fn next_event(&mut self) -> u64 {
        assert_eq!(self.exs.len(), 0);
        self.max += 1;
        self.max
    }

    /// Adds an event to the set.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut above_exset = AboveExSet::new();
    ///
    /// above_exset.add_event(1);
    /// assert!(above_exset.is_event(&1));
    /// assert!(!above_exset.is_event(&2));
    ///
    /// above_exset.add_event(3);
    /// assert!(above_exset.is_event(&1));
    /// assert!(!above_exset.is_event(&2));
    /// assert!(above_exset.is_event(&3));
    ///
    /// above_exset.add_event(2);
    /// assert!(above_exset.is_event(&1));
    /// assert!(above_exset.is_event(&2));
    /// assert!(above_exset.is_event(&3));
    /// ```
    fn add_event(&mut self, event: u64) {
        if event == self.max + 1 {
            // this event is now the new max
            self.max = event;

            // maybe compress
            self.try_compress();
        } else if event > self.max + 1 {
            // add as an extra
            self.exs.insert(event);
        }
        // else it's already an event
    }

    /// Checks if an event is part of the set.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut above_exset = AboveExSet::new();
    /// let event = above_exset.next_event();
    /// assert!(above_exset.is_event(&event));
    ///
    /// above_exset.add_event(3);
    /// assert!(!above_exset.is_event(&2));
    /// assert!(above_exset.is_event(&3));
    /// ```
    fn is_event(&self, event: &u64) -> bool {
        *event <= self.max || self.exs.contains(event)
    }

    /// Returns all events seen as a tuple.
    /// The first component is the highest event seen, while the second is a
    /// vector with the exceptions (in no specific order).
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut above_exset = AboveExSet::new();
    ///
    /// above_exset.add_event(1);
    /// assert_eq!(above_exset.events(), (1, vec![]));
    ///
    /// above_exset.add_event(3);
    /// assert_eq!(above_exset.events(), (1, vec![3]));
    ///
    /// above_exset.add_event(2);
    /// assert_eq!(above_exset.events(), (3, vec![]));
    ///
    /// above_exset.add_event(4);
    /// assert_eq!(above_exset.events(), (4, vec![]));
    ///
    /// above_exset.add_event(6);
    /// assert_eq!(above_exset.events(), (4, vec![6]));
    /// ```
    fn events(&self) -> (u64, Vec<u64>) {
        (self.max, self.exs.clone().into_iter().collect())
    }

    /// Merges `other` `AboveExSet` into `self`.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut above_exset = AboveExSet::new();
    /// above_exset.add_event(1);
    /// above_exset.add_event(3);
    /// above_exset.add_event(4);
    /// assert_eq!(above_exset.events(), (1, vec![3, 4]));
    ///
    /// above_exset.join(&AboveExSet::from_event(3));
    /// assert_eq!(above_exset.events(), (1, vec![3, 4]));
    ///
    /// above_exset.join(&AboveExSet::from_event(5));
    /// assert_eq!(above_exset.events(), (1, vec![3, 4, 5]));
    ///
    /// let mut other = AboveExSet::new();
    /// other.add_event(2);
    /// other.add_event(7);
    /// above_exset.join(&other);
    /// assert_eq!(above_exset.events(), (5, vec![7]));
    /// ```
    fn join(&mut self, other: &Self) {
        // the new max value is the max of both max values
        self.max = std::cmp::max(self.max, other.max);

        // add all extras as extras
        other.exs.iter().for_each(|ex| {
            self.exs.insert(*ex);
        });

        // maybe compress
        self.try_compress();
    }
}

impl AboveExSet {
    /// Tries to set a new max contiguous event.
    fn try_compress(&mut self) {
        // bind the borrow to a new variable, as suggested here:
        // - https://github.com/rust-lang/rust/issues/19004#issuecomment-63220141
        let max = &mut self.max;

        // only keep in extras those that can't be compressed
        self.exs = self
            .exs
            .iter()
            .filter(|&&extra| {
                if extra == *max + 1 {
                    // we have a new max
                    *max = extra;

                    // don't keep it in extras
                    false
                } else {
                    // keep it in extras
                    true
                }
            })
            .cloned()
            .collect();
    }

    /// Creates a new instance from the highest contiguous event, and a sequence
    /// of extra events.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let above_exset = AboveExSet::from(0, vec![2, 4, 5]);
    /// assert!(!above_exset.is_event(&1));
    /// assert!(above_exset.is_event(&2));
    /// assert!(!above_exset.is_event(&3));
    /// assert!(above_exset.is_event(&4));
    /// assert!(above_exset.is_event(&5));
    /// assert!(!above_exset.is_event(&6));
    /// ```
    pub fn from<I: IntoIterator<Item = u64>>(max: u64, iter: I) -> Self {
        AboveExSet {
            max,
            exs: BTreeSet::from_iter(iter),
        }
    }
}

pub struct IntoIter {
    // Last contiguous value returned by the iterator
    current: u64,
    // Last contiguous value that should be returned by the iterator
    max: u64,
    // Set of exceptions to be skipped by the iterator
    exs: std::collections::btree_set::IntoIter<u64>,
}

impl Iterator for IntoIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.max {
            // we've reached the last contiguous, just call next on the extras
            // iterator
            self.exs.next()
        } else {
            // compute next value
            self.current += 1;
            Some(self.current)
        }
    }
}

impl IntoIterator for AboveExSet {
    type Item = u64;
    type IntoIter = IntoIter;

    /// Returns a `AboveExSet` into iterator with all events from lowest to
    /// highest.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut above_exset = AboveExSet::new();
    /// above_exset.add_event(3);
    /// above_exset.add_event(5);
    ///
    /// let mut iter = above_exset.into_iter();
    /// assert_eq!(iter.next(), Some(3));
    /// assert_eq!(iter.next(), Some(5));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            current: 0,
            max: self.max,
            exs: self.exs.into_iter(),
        }
    }
}
