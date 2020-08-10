//! This module contains an implementation of an above-extra set.
//!
//! # Examples
//! ```
//! use threshold::*;
//!
//! let mut above_exset = AboveExSet::new();
//! assert_eq!(above_exset.next_event(), 1);
//! assert!(above_exset.is_event(1));
//! assert!(!above_exset.is_event(2));
//!
//! let other = AboveExSet::from_event(3);
//! assert!(!other.is_event(1));
//! assert!(!other.is_event(2));
//! assert!(other.is_event(3));
//!
//! above_exset.join(&other);
//! assert!(above_exset.is_event(1));
//! assert!(!above_exset.is_event(2));
//! assert!(above_exset.is_event(3));
//! ```

use crate::EventSet;
use serde::{Deserialize, Serialize};
use std::cmp;
use std::cmp::Ordering;
use std::collections::btree_set::{self, BTreeSet};
use std::collections::HashSet;
use std::fmt;
use std::iter::FromIterator;

#[derive(Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct AboveExSet {
    // Highest contiguous event seen
    max: u64,
    // Set of extra events above the highest (sorted ASC)
    exs: HashSet<u64>,
}

impl EventSet for AboveExSet {
    type EventIter = EventIter;

    /// Returns a new `AboveExSet` instance.
    fn new() -> Self {
        AboveExSet {
            max: 0,
            exs: HashSet::new(),
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
        debug_assert!(self.exs.is_empty());
        self.max += 1;
        self.max
    }

    /// Adds an event to the set.
    /// Returns `true` if it's a new event.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut above_exset = AboveExSet::new();
    ///
    /// above_exset.add_event(1);
    /// assert!(above_exset.is_event(1));
    /// assert!(!above_exset.is_event(2));
    ///
    /// above_exset.add_event(3);
    /// assert!(above_exset.is_event(1));
    /// assert!(!above_exset.is_event(2));
    /// assert!(above_exset.is_event(3));
    ///
    /// above_exset.add_event(2);
    /// assert!(above_exset.is_event(1));
    /// assert!(above_exset.is_event(2));
    /// assert!(above_exset.is_event(3));
    /// ```
    fn add_event(&mut self, event: u64) -> bool {
        let next_max = self.max + 1;
        match event.cmp(&next_max) {
            Ordering::Equal => {
                // this event is now the new max
                self.max = event;

                // maybe compress
                self.try_compress();

                // new event, so `true`
                true
            }
            Ordering::Greater => {
                // add as an extra. the result is the same as the result of the
                // insert in the extras:
                // - if it's a new extra, then it's also a new event
                self.exs.insert(event)
            }
            Ordering::Less => {
                // else it's already an event
                false
            }
        }
    }

    /// Adds a range of events to the set.
    fn add_event_range(&mut self, start: u64, end: u64) -> bool {
        if start <= self.max + 1 && end > self.max {
            // the end of the range is now the new max
            self.max = end;

            // maybe compress
            self.try_compress();

            // new event, so `true`
            true
        } else if start > self.max + 1 {
            // add all events as extra
            self.exs.extend(start..=end);
            true
        } else {
            // else all events are already an event
            false
        }
    }

    /// Checks if an event is part of the set.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut above_exset = AboveExSet::new();
    /// let event = above_exset.next_event();
    /// assert!(above_exset.is_event(event));
    ///
    /// above_exset.add_event(3);
    /// assert!(!above_exset.is_event(2));
    /// assert!(above_exset.is_event(3));
    /// ```
    fn is_event(&self, event: u64) -> bool {
        event <= self.max || self.exs.contains(&event)
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
        let mut exs: Vec<_> = self.exs.clone().into_iter().collect();
        exs.sort_unstable();
        (self.max, exs)
    }

    /// Returns the frontier (the highest contiguous event seen).
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut above_exset = AboveExSet::new();
    /// assert_eq!(above_exset.frontier(), 0);
    ///
    /// above_exset.add_event(1);
    /// assert_eq!(above_exset.frontier(), 1);
    ///
    /// above_exset.add_event(3);
    /// assert_eq!(above_exset.frontier(), 1);
    ///
    /// above_exset.add_event(2);
    /// assert_eq!(above_exset.frontier(), 3);
    ///
    /// above_exset.add_event(4);
    /// assert_eq!(above_exset.frontier(), 4);
    ///
    /// above_exset.add_event(6);
    /// assert_eq!(above_exset.frontier(), 4);
    /// ```
    fn frontier(&self) -> u64 {
        self.max
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
        self.max = cmp::max(self.max, other.max);

        // add all extras as extras
        other.exs.iter().for_each(|ex| {
            self.exs.insert(*ex);
        });

        // maybe compress
        self.try_compress();
    }

    fn meet(&mut self, _other: &Self) {
        todo!("AboveExSet::meet not yet implemented")
    }

    /// Returns a `AboveExSet` event iterator with all events from lowest to
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
    /// let mut iter = above_exset.event_iter();
    /// assert_eq!(iter.next(), Some(3));
    /// assert_eq!(iter.next(), Some(5));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn event_iter(self) -> Self::EventIter {
        EventIter {
            current: 0,
            max: self.max,
            exs: BTreeSet::from_iter(self.exs).into_iter(),
        }
    }
}

impl AboveExSet {
    /// Tries to set a new max contiguous event.
    fn try_compress(&mut self) {
        // only keep in extras those that can't be compressed
        while self.exs.remove(&(self.max + 1)) {
            self.max = self.max + 1;
        }
    }

    /// Creates a new instance from the highest contiguous event, and a sequence
    /// of extra events.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let above_exset = AboveExSet::from(0, vec![2, 4, 5]);
    /// assert!(!above_exset.is_event(1));
    /// assert!(above_exset.is_event(2));
    /// assert!(!above_exset.is_event(3));
    /// assert!(above_exset.is_event(4));
    /// assert!(above_exset.is_event(5));
    /// assert!(!above_exset.is_event(6));
    /// ```
    pub fn from<I: IntoIterator<Item = u64>>(max: u64, iter: I) -> Self {
        AboveExSet {
            max,
            exs: HashSet::from_iter(iter),
        }
    }
}

pub struct EventIter {
    // Last contiguous value returned by the iterator
    current: u64,
    // Last contiguous value that should be returned by the iterator
    max: u64,
    // Iterator of extras
    exs: btree_set::IntoIter<u64>,
}

impl Iterator for EventIter {
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

impl fmt::Debug for AboveExSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.exs.is_empty() {
            write!(f, "{}", self.max)
        } else {
            let exs: std::collections::BTreeSet<_> = self.exs.iter().collect();
            write!(f, "({} + {:?})", self.max, exs)
        }
    }
}
