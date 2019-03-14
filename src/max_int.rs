//! This module contains an implementation of a max int.
//!
//! # Examples
//! ```
//! use threshold::*;
//! ```

use crate::EventSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaxInt {
    // Highest event seen
    event: u64,
}

impl EventSet for MaxInt {
    /// Returns a new `MaxInt` instance.
    fn new() -> Self {
        MaxInt { event: 0 }
    }

    /// Creates a new instance from `event`.
    fn from_event(event: u64) -> Self {
        MaxInt { event }
    }

    /// Generates the next event.
    fn next_event(&mut self) -> u64 {
        self.event += 1;
        self.event
    }

    /// Adds an event to the set.
    fn add_event(&mut self, event: u64) {
        self.event = std::cmp::max(self.event, event);
    }

    /// Checks if an event is part of the set.
    fn is_event(&self, event: &u64) -> bool {
        *event <= self.event
    }

    /// Returns all events seen.
    fn events(&self) -> (u64, Vec<u64>) {
        (self.event, vec![])
    }

    /// Merges `other` `MaxInt` into `self`.
    fn join(&mut self, other: &Self) {
        self.add_event(other.event);
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

impl IntoIterator for MaxInt {
    type Item = u64;
    type IntoIter = IntoIter;

    /// Returns a `MaxInt` into iterator with all events from lowest to highest.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut max_int = MaxInt::new();
    /// max_int.add_event(3);
    ///
    /// let mut iter = max_int.into_iter();
    /// assert_eq!(iter.next(), Some(1));
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), Some(3));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            current: 0,
            max: self.event,
        }
    }
}
