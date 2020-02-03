//! This module contains an implementation of a max set.
//!
//! # Examples
//! ```
//! use threshold::*;
//!
//! let mut maxset = MaxSet::new();
//! assert_eq!(maxset.next_event(), 1);
//! assert!(maxset.is_event(1));
//! assert!(!maxset.is_event(2));
//!
//! let other = MaxSet::from_event(3);
//! assert!(other.is_event(1));
//! assert!(other.is_event(2));
//! assert!(other.is_event(3));
//!
//! maxset.join(&other);
//! assert!(maxset.is_event(3));
//! ```

use crate::EventSet;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MaxSet {
    // Highest event seen
    max: u64,
}

impl EventSet for MaxSet {
    type EventIter = EventIter;

    /// Returns a new `MaxSet` instance.
    #[inline]
    fn new() -> Self {
        MaxSet { max: 0 }
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
    #[inline]
    fn next_event(&mut self) -> u64 {
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
    /// let mut maxset = MaxSet::new();
    /// assert!(!maxset.is_event(9));
    /// assert!(!maxset.is_event(10));
    ///
    /// maxset.add_event(10);
    /// assert!(maxset.is_event(9));
    /// assert!(maxset.is_event(10));
    /// ```
    #[inline]
    fn add_event(&mut self, event: u64) -> bool {
        if event <= self.max {
            false
        } else {
            self.max = event;
            true
        }
    }

    /// Adds a range of events to the set.
    /// Returns `true` if a new event was added.
    ///
    /// In the case of `MaxSet` we have that:
    /// - `add_event_range(start, end) == add_event(end)`
    #[inline]
    fn add_event_range(&mut self, start: u64, end: u64) -> bool {
        assert!(start <= end);
        self.add_event(end)
    }

    /// Checks if an event is part of the set.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut maxset = MaxSet::new();
    /// let event = maxset.next_event();
    /// assert!(maxset.is_event(event));
    /// ```
    #[inline]
    fn is_event(&self, event: u64) -> bool {
        event <= self.max
    }

    /// Returns all events seen.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut maxset = MaxSet::new();
    /// maxset.add_event(2);
    /// maxset.add_event(4);
    /// assert_eq!(maxset.events(), (4, vec![]));
    /// ```
    #[inline]
    fn events(&self) -> (u64, Vec<u64>) {
        (self.max, vec![])
    }

    /// Returns the frontier (the highest contiguous event seen).
    /// For a `MaxSet`, this is not necessarily the highest contiguous event,
    /// but simply the highest event.
    /// For exact `EventSet` representations that will actually return the
    /// highest contiguous event, see `AboveExSet` and `BelowExSet`.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut maxset = MaxSet::new();
    /// assert_eq!(maxset.frontier(), 0);
    ///
    /// maxset.add_event(1);
    /// assert_eq!(maxset.frontier(), 1);
    ///
    /// maxset.add_event(3);
    /// assert_eq!(maxset.frontier(), 3);
    ///
    /// maxset.add_event(2);
    /// assert_eq!(maxset.frontier(), 3);
    ///
    /// maxset.add_event(4);
    /// assert_eq!(maxset.frontier(), 4);
    ///
    /// maxset.add_event(6);
    /// assert_eq!(maxset.frontier(), 6);
    /// ```
    #[inline]
    fn frontier(&self) -> u64 {
        self.max
    }

    /// Merges `other` `MaxSet` into `self`.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut maxset = MaxSet::from_event(10);
    /// assert!(!maxset.is_event(20));
    ///
    /// maxset.join(&MaxSet::from_event(20));
    /// assert!(maxset.is_event(20));
    /// ```
    #[inline]
    fn join(&mut self, other: &Self) {
        self.add_event(other.max);
    }

    /// Returns a `MaxSet` event iterator with all events from lowest to
    /// highest.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut maxset = MaxSet::new();
    /// maxset.add_event(3);
    ///
    /// let mut iter = maxset.event_iter();
    /// assert_eq!(iter.next(), Some(1));
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), Some(3));
    /// assert_eq!(iter.next(), None);
    /// ```
    #[inline]
    fn event_iter(self) -> Self::EventIter {
        EventIter {
            current: 0,
            max: self.max,
        }
    }
}

pub struct EventIter {
    // Last value returned by the iterator
    current: u64,
    // Last value that should be returned by the iterator
    max: u64,
}

impl Iterator for EventIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.max {
            // we've reached the end of the iterator
            None
        } else {
            // compute next value and return it
            self.current += 1;
            Some(self.current)
        }
    }
}

impl fmt::Debug for MaxSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.max)
    }
}
