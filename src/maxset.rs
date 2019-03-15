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
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let maxset = MaxSet::from_event(10);
    /// assert!(maxset.is_event(&10));
    /// ```
    fn from_event(event: u64) -> Self {
        MaxSet { max: event }
    }

    /// Generates the next event.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut maxset = MaxSet::new();
    /// assert_eq!(maxset.next_event(), 1);
    /// assert_eq!(maxset.next_event(), 2);
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
    /// let mut maxset = MaxSet::new();
    /// assert!(!maxset.is_event(&9));
    /// assert!(!maxset.is_event(&10));
    ///
    /// maxset.add_event(10);
    /// assert!(maxset.is_event(&9));
    /// assert!(maxset.is_event(&10));
    /// ```
    fn add_event(&mut self, event: u64) {
        self.max = std::cmp::max(self.max, event);
    }

    /// Checks if an event is part of the set.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut maxset = MaxSet::new();
    /// let event = maxset.next_event();
    /// assert!(maxset.is_event(&event));
    /// ```
    fn is_event(&self, event: &u64) -> bool {
        *event <= self.max
    }

    /// Returns all events seen.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let maxset = MaxSet::from_event(10);
    /// assert_eq!(maxset.events(), (10, vec![]));
    /// ```
    fn events(&self) -> (u64, Vec<u64>) {
        (self.max, vec![])
    }

    /// Merges `other` `MaxSet` into `self`.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut maxset = MaxSet::from_event(10);
    /// assert!(!maxset.is_event(&20));
    ///
    /// maxset.join(&MaxSet::from_event(20));
    /// assert!(maxset.is_event(&20));
    /// ```
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
