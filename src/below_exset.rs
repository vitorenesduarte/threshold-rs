//! This module contains an implementation of a below-exception set.
//!
//! # Examples
//! ```
//! use threshold::*;
//!
//! let mut below_exset = BelowExSet::new();
//! assert_eq!(below_exset.next_event(), 1);
//! assert!(below_exset.is_event(1));
//! assert!(!below_exset.is_event(2));
//!
//! let other = BelowExSet::from_event(3);
//! assert!(!other.is_event(1));
//! assert!(!other.is_event(2));
//! assert!(other.is_event(3));
//!
//! below_exset.join(&other);
//! assert!(below_exset.is_event(1));
//! assert!(!below_exset.is_event(2));
//! assert!(below_exset.is_event(3));
//! ```

use crate::EventSet;
use std::cmp::{self, Ordering};
use std::collections::HashSet;
use std::fmt;
use std::iter::FromIterator;

#[derive(Clone, PartialEq, Eq, Default)]
pub struct BelowExSet {
    // Highest event seen
    max: u64,
    // Set of exceptions
    exs: HashSet<u64>,
}

impl EventSet for BelowExSet {
    type EventIter = EventIter;

    /// Returns a new `BelowExSet` instance.
    fn new() -> Self {
        BelowExSet {
            max: 0,
            exs: HashSet::new(),
        }
    }

    /// Generates the next event.
    /// There should be no exceptions when calling this.
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
    /// assert!(below_exset.is_event(1));
    /// assert!(!below_exset.is_event(2));
    ///
    /// below_exset.add_event(3);
    /// assert!(below_exset.is_event(1));
    /// assert!(!below_exset.is_event(2));
    /// assert!(below_exset.is_event(3));
    ///
    /// below_exset.add_event(2);
    /// assert!(below_exset.is_event(1));
    /// assert!(below_exset.is_event(2));
    /// assert!(below_exset.is_event(3));
    /// ```
    fn add_event(&mut self, event: u64) -> bool {
        match event.cmp(&self.max) {
            Ordering::Less => {
                // remove from exceptions (it might not be an exception though).
                // the result is the same as the result of the remove in the
                // exceptions:
                // - if it was an exception, then it's also a new event
                self.exs.remove(&event)
            }
            Ordering::Greater => {
                // this event is now the new max, which might create exceptions
                for new_ex in self.max + 1..event {
                    self.exs.insert(new_ex);
                }
                self.max = event;
                // new event, so `true`
                true
            }
            Ordering::Equal => {
                // nothing to do since it is already an event
                false
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
    /// assert!(below_exset.is_event(event));
    ///
    /// below_exset.add_event(3);
    /// assert!(!below_exset.is_event(2));
    /// assert!(below_exset.is_event(3));
    /// ```
    fn is_event(&self, event: u64) -> bool {
        event <= self.max && !self.exs.contains(&event)
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

    /// Returns the frontier (the highest contiguous event seen).
    ///
    /// __Note:__ this method's implementation will sort all exceptions on each
    /// call, and with that, the performance will not be great. If this
    /// becomes a problem, we could cache the frontier (as in `AboveExSet`)
    /// so that it doesn't have to be computed here on each call.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut below_exset = BelowExSet::new();
    /// assert_eq!(below_exset.frontier(), 0);
    ///
    /// below_exset.add_event(1);
    /// assert_eq!(below_exset.frontier(), 1);
    ///
    /// below_exset.add_event(3);
    /// assert_eq!(below_exset.frontier(), 1);
    ///
    /// below_exset.add_event(2);
    /// assert_eq!(below_exset.frontier(), 3);
    ///
    /// below_exset.add_event(4);
    /// assert_eq!(below_exset.frontier(), 4);
    ///
    /// below_exset.add_event(6);
    /// assert_eq!(below_exset.frontier(), 4);
    /// ```
    fn frontier(&self) -> u64 {
        // if there are no exceptions, then the highest contiguous event is
        // self.max otherwise, it's the smallest exception - 1
        if self.exs.is_empty() {
            self.max
        } else {
            // sort exceptions
            let mut exs: Vec<_> = self.exs.iter().collect();
            exs.sort_unstable();

            // return the smallest one -1
            (**exs.iter().next().unwrap()) - 1
        }
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
        let before = self.clone();

        // the new exceptions are a subset of the union of exceptions sets
        // - this means that the join does not create new exceptions
        //
        // keep the local exceptions that are not remote events
        self.exs.retain(|ex| !other.is_event(*ex));

        // keep the remote exceptions that are not local events
        other
            .exs
            .iter()
            .filter(|&&ex| !before.is_event(ex))
            .for_each(|&ex| {
                self.exs.insert(ex);
            });

        // the new max value is the max of both max values
        self.max = cmp::max(self.max, other.max);
    }

    /// Returns a `BelowExSet` event iterator with all events from lowest to
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
    /// let mut iter = below_exset.event_iter();
    /// assert_eq!(iter.next(), Some(3));
    /// assert_eq!(iter.next(), Some(5));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn event_iter(self) -> Self::EventIter {
        EventIter {
            current: 0,
            max: self.max,
            exs: self.exs,
        }
    }
}

impl BelowExSet {
    /// Creates a new instance from the highest event, and a sequence of
    /// exceptions.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let below_exset = BelowExSet::from(5, vec![1, 3]);
    /// assert!(!below_exset.is_event(1));
    /// assert!(below_exset.is_event(2));
    /// assert!(!below_exset.is_event(3));
    /// assert!(below_exset.is_event(4));
    /// assert!(below_exset.is_event(5));
    /// assert!(!below_exset.is_event(6));
    /// ```
    pub fn from<I: IntoIterator<Item = u64>>(max: u64, iter: I) -> Self {
        BelowExSet {
            max,
            exs: HashSet::from_iter(iter),
        }
    }
}

pub struct EventIter {
    // Last value returned by the iterator
    current: u64,
    // Last value that should be returned by the iterator
    max: u64,
    // Set of exceptions to be skipped by the iterator
    exs: HashSet<u64>,
}

impl Iterator for EventIter {
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
                self.next()
            } else {
                // otherwise, return it
                Some(self.current)
            }
        }
    }
}

impl fmt::Debug for BelowExSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.exs.is_empty() {
            write!(f, "{}", self.max)
        } else {
            write!(f, "({} - {:?})", self.max, self.exs)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range() {
        // add event 1 and 2 to eset
        let mut eset = BelowExSet::new();
        eset.add_event(1);
        eset.add_event(2);

        // create range
        let start = 1;
        let end = 2;

        // check it's the same
        assert_eq!(eset, BelowExSet::from_event_range(start, end));
    }
}
