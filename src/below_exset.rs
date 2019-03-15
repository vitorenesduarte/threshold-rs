//! This module contains an implementation of a below-exception set.
//!
//! # Examples
//! ```
//! use threshold::*;
//!
//! let mut below_exset = BelowExSet::new();
//! assert_eq!(below_exset.next_event(), 1);
//! assert!(below_exset.is_event(&1));
//! assert!(!below_exset.is_event(&2));
//!
//! let other = BelowExSet::from_event(3);
//! assert!(!other.is_event(&1));
//! assert!(!other.is_event(&2));
//! assert!(other.is_event(&3));
//!
//! below_exset.join(&other);
//! assert!(below_exset.is_event(&1));
//! assert!(!below_exset.is_event(&2));
//! assert!(below_exset.is_event(&3));
//! ```

use crate::EventSet;
use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BelowExSet {
    // Highest event seen
    max: u64,
    // Set of exceptions
    exs: HashSet<u64>,
}

impl EventSet for BelowExSet {
    /// Returns a new `BelowExSet` instance.
    fn new() -> Self {
        BelowExSet {
            max: 0,
            exs: HashSet::new(),
        }
    }

    /// Creates a new instance from `event`.
    /// All events smaller than `event` will become exceptions.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let below_exset = BelowExSet::from_event(3);
    /// assert!(!below_exset.is_event(&1));
    /// assert!(!below_exset.is_event(&2));
    /// assert!(below_exset.is_event(&3));
    /// assert!(!below_exset.is_event(&4));
    /// ```
    fn from_event(event: u64) -> Self {
        let exs = (1..event).collect();
        BelowExSet { max: event, exs }
    }

    /// Generates the next event.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut below_exset = BelowExSet::new();
    /// assert_eq!(below_exset.next_event(), 1);
    /// assert_eq!(below_exset.next_event(), 2);
    /// ```
    fn next_event(&mut self) -> u64 {
        self.max += 1;
        self.max
    }

    /// Adds an event to the set.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut below_exset = BelowExSet::new();
    ///
    /// below_exset.add_event(1);
    /// assert!(below_exset.is_event(&1));
    /// assert!(!below_exset.is_event(&2));
    ///
    /// below_exset.add_event(3);
    /// assert!(below_exset.is_event(&1));
    /// assert!(!below_exset.is_event(&2));
    /// assert!(below_exset.is_event(&3));
    ///
    /// below_exset.add_event(2);
    /// assert!(below_exset.is_event(&1));
    /// assert!(below_exset.is_event(&2));
    /// assert!(below_exset.is_event(&3));
    /// ```
    fn add_event(&mut self, event: u64) {
        match event.cmp(&self.max) {
            Ordering::Less => {
                // remove from exceptions
                self.exs.remove(&event);
            }
            Ordering::Greater => {
                // this event is now the new max, which might create exceptions
                for new_ex in self.max + 1..event {
                    self.exs.insert(new_ex);
                }
                self.max = event;
            }
            Ordering::Equal => {
                // nothing to do, already an event
            }
        }
    }

    /// Checks if an event is part of the set.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut below_exset = BelowExSet::new();
    /// let event = below_exset.next_event();
    /// assert!(below_exset.is_event(&event));
    ///
    /// below_exset.add_event(3);
    /// assert!(!below_exset.is_event(&2));
    /// assert!(below_exset.is_event(&3));
    /// ```
    fn is_event(&self, event: &u64) -> bool {
        *event <= self.max && !self.exs.contains(event)
    }

    /// Returns all events seen as a tuple.
    /// The first component is the highest event seen, while the second is a
    /// vector with the exceptions (in no specific order).
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut below_exset = BelowExSet::new();
    ///
    /// below_exset.add_event(1);
    /// assert_eq!(below_exset.events(), (1, vec![]));
    ///
    /// below_exset.add_event(3);
    /// assert_eq!(below_exset.events(), (3, vec![2]));
    ///
    /// below_exset.add_event(2);
    /// assert_eq!(below_exset.events(), (3, vec![]));
    ///
    /// below_exset.add_event(4);
    /// assert_eq!(below_exset.events(), (4, vec![]));
    ///
    /// below_exset.add_event(6);
    /// assert_eq!(below_exset.events(), (6, vec![5]));
    /// ```
    fn events(&self) -> (u64, Vec<u64>) {
        (self.max, self.exs.clone().into_iter().collect())
    }

    /// Merges `other` `BelowExSet` into `self`.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut below_exset = BelowExSet::new();
    /// below_exset.add_event(1);
    /// below_exset.add_event(3);
    /// below_exset.add_event(4);
    /// assert_eq!(below_exset.events(), (4, vec![2]));
    ///
    /// below_exset.join(&BelowExSet::from_event(3));
    /// assert_eq!(below_exset.events(), (4, vec![2]));
    ///
    /// below_exset.join(&BelowExSet::from_event(5));
    /// assert_eq!(below_exset.events(), (5, vec![2]));
    ///
    /// let mut other = BelowExSet::new();
    /// other.add_event(2);
    /// other.add_event(7);
    /// below_exset.join(&other);
    /// assert_eq!(below_exset.events(), (7, vec![6]));
    /// ```
    fn join(&mut self, other: &Self) {
        //
        // - the new max value is the max of both max values
        // - the new exceptions are a subset of the union of exceptions sets
        //  - this means we don't create new exceptions
        //  - of those exceptions in the union, we keep the ones that: 1. are
        //    bigger than the min of both max values OR 2. are in the
        //    intersection of both exceptions sets
        //  - in a more formal way, the final set of exceptions is given by:
        // {ex ∈ A.exs ∪ B.exs | ex > min(A.max, B.max) \/ ex ∈ A.exs ∩ B.exs }
        let exs_before = self.exs.clone();

        // keep local exceptions that are:
        // - higher than the `other` highest event
        // - also an exception
        self.exs
            .retain(|&ex| ex > other.max || other.exs.contains(&ex));

        // add remote exception that are:
        // - higher than current max
        // - part of `exceptions_before`
        for &ex in other.exs.iter() {
            if ex > self.max || exs_before.contains(&ex) {
                self.exs.insert(ex);
            }
        }

        self.max = std::cmp::max(self.max, other.max);
    }
}

pub struct IntoIter {
    // Last value returned by the iterator
    current: u64,
    // Last value that should be returned by the iterator
    max: u64,
    // Set of exceptions to be skipped by the iterator
    exs: HashSet<u64>,
}

impl Iterator for IntoIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.max {
            // we've reached the end of the iterator
            None
        } else {
            // compute next value
            self.current += 1;

            if self.exs.contains(&self.current) {
                // if the next value is an exception, skip it
                return self.next();
            } else {
                // otherwise, return it
                Some(self.current)
            }
        }
    }
}

impl IntoIterator for BelowExSet {
    type Item = u64;
    type IntoIter = IntoIter;

    /// Returns a `BelowExSet` into iterator with all events from lowest to
    /// highest.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut below_exset = BelowExSet::new();
    /// below_exset.add_event(3);
    /// below_exset.add_event(5);
    ///
    /// let mut iter = below_exset.into_iter();
    /// assert_eq!(iter.next(), Some(3));
    /// assert_eq!(iter.next(), Some(5));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            current: 0,
            max: self.max,
            exs: self.exs.clone(),
        }
    }
}
