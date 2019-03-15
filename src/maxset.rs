//! This module contains an implementation of a max set.
//!
//! # Examples
//! ```
//! use threshold::*;
//! ```

use crate::EventSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaxSet {
    // Highest event seen
    max: u64,
}

impl EventSet for MaxSet {
    /// Returns a new `MaxSet` instance.
    fn new() -> Self {
        MaxSet { max: 0 }
    }

    /// Creates a new instance from `event`.
    fn from_event(event: u64) -> Self {
        MaxSet { max: event }
    }

    /// Generates the next event.
    fn next_event(&mut self) -> u64 {
        self.max += 1;
        self.max
    }

    /// Adds an event to the set.
    fn add_event(&mut self, event: u64) {
        self.max = std::cmp::max(self.max, event);
    }

    /// Checks if an event is part of the set.
    fn is_event(&self, event: &u64) -> bool {
        *event <= self.max
    }

    /// Returns all events seen.
    fn events(&self) -> (u64, Vec<u64>) {
        (self.max, vec![])
    }

    /// Merges `other` `MaxSet` into `self`.
    fn join(&mut self, other: &Self) {
        self.add_event(other.max);
    }
}

pub struct IntoIter {
    // Last value returned by the iterator
    current: u64,
    // Last value that should be returned by the iterator
    max: u64,
}

impl Iterator for IntoIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.max {
            None
        } else {
            self.current += 1;
            Some(self.current)
        }
    }
}

impl IntoIterator for MaxSet {
    type Item = u64;
    type IntoIter = IntoIter;

    /// Returns a `MaxSet` into iterator with all events from lowest to highest.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut maxset = MaxSet::new();
    /// maxset.add_event(3);
    ///
    /// let mut iter = maxset.into_iter();
    /// assert_eq!(iter.next(), Some(1));
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), Some(3));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            current: 0,
            max: self.max,
        }
    }
}
