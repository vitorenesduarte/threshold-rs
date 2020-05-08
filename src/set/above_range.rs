//! This module contains an implementation of an above-extra-range set.
//!
//! # Examples
//! ```
//! use threshold::*;
//!
//! let mut above_range_set = AboveRangeSet::new();
//! assert_eq!(above_range_set.next_event(), 1);
//! assert!(above_range_set.is_event(1));
//! assert!(!above_range_set.is_event(2));
//!
//! let other = AboveRangeSet::from_event(3);
//! assert!(!other.is_event(1));
//! assert!(!other.is_event(2));
//! assert!(other.is_event(3));
//!
//! above_range_set.join(&other);
//! assert!(above_range_set.is_event(1));
//! assert!(!above_range_set.is_event(2));
//! assert!(above_range_set.is_event(3));
//! ```

use crate::EventSet;
use serde::{Deserialize, Serialize};
use std::cmp;
use std::collections::BTreeMap;
use std::fmt;

#[derive(Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct AboveRangeSet {
    // Highest contiguous event seen
    max: u64,
    // Mapping from start of the range to its end (sorted ASC)
    ranges: Ranges,
}

#[derive(Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Ranges {
    // Mapping from start of the range to its end (sorted ASC)
    ranges: BTreeMap<u64, u64>,
}

impl EventSet for AboveRangeSet {
    type EventIter = EventIter;

    /// Returns a new `AboveRangeSet` instance.
    fn new() -> Self {
        AboveRangeSet {
            max: 0,
            ranges: Ranges::new(),
        }
    }

    /// Generates the next event.
    /// There should be no extra ranges when calling this.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut above_range_set = AboveRangeSet::new();
    /// assert_eq!(above_range_set.next_event(), 1);
    /// assert_eq!(above_range_set.next_event(), 2);
    /// ```
    fn next_event(&mut self) -> u64 {
        debug_assert!(self.ranges.is_empty());
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
    /// let mut above_range_set = AboveRangeSet::new();
    ///
    /// above_range_set.add_event(1);
    /// assert!(above_range_set.is_event(1));
    /// assert!(!above_range_set.is_event(2));
    ///
    /// above_range_set.add_event(3);
    /// assert!(above_range_set.is_event(1));
    /// assert!(!above_range_set.is_event(2));
    /// assert!(above_range_set.is_event(3));
    ///
    /// above_range_set.add_event(2);
    /// assert!(above_range_set.is_event(1));
    /// assert!(above_range_set.is_event(2));
    /// assert!(above_range_set.is_event(3));
    /// ```
    fn add_event(&mut self, event: u64) -> bool {
        if event == self.max + 1 {
            // this event is now the new max
            self.max = event;

            // maybe compress
            self.try_compress();

            // new event, so `true`
            true
        } else if event > self.max + 1 {
            // add as a range: assumes it's a new range
            self.ranges.add(event, event);
            true
        } else {
            // else it's already an event
            false
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
            // add as a range: assumes it's a new range
            self.ranges.add(start, end);
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
    /// let mut above_range_set = AboveRangeSet::new();
    /// let event = above_range_set.next_event();
    /// assert!(above_range_set.is_event(event));
    ///
    /// above_range_set.add_event(3);
    /// assert!(!above_range_set.is_event(2));
    /// assert!(above_range_set.is_event(3));
    /// ```
    fn is_event(&self, event: u64) -> bool {
        event <= self.max || self.ranges.contains(&event)
    }

    /// Returns all events seen as a tuple.
    /// The first component is the highest event seen, while the second is a
    /// vector with the exceptions (in no specific order).
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut above_range_set = AboveRangeSet::new();
    ///
    /// above_range_set.add_event(1);
    /// assert_eq!(above_range_set.events(), (1, vec![]));
    ///
    /// above_range_set.add_event(3);
    /// assert_eq!(above_range_set.events(), (1, vec![3]));
    ///
    /// above_range_set.add_event(2);
    /// assert_eq!(above_range_set.events(), (3, vec![]));
    ///
    /// above_range_set.add_event(4);
    /// assert_eq!(above_range_set.events(), (4, vec![]));
    ///
    /// above_range_set.add_event(6);
    /// assert_eq!(above_range_set.events(), (4, vec![6]));
    /// ```
    fn events(&self) -> (u64, Vec<u64>) {
        (self.max, self.ranges.clone().event_iter().collect())
    }

    /// Returns the frontier (the highest contiguous event seen).
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut above_range_set = AboveRangeSet::new();
    /// assert_eq!(above_range_set.frontier(), 0);
    ///
    /// above_range_set.add_event(1);
    /// assert_eq!(above_range_set.frontier(), 1);
    ///
    /// above_range_set.add_event(3);
    /// assert_eq!(above_range_set.frontier(), 1);
    ///
    /// above_range_set.add_event(2);
    /// assert_eq!(above_range_set.frontier(), 3);
    ///
    /// above_range_set.add_event(4);
    /// assert_eq!(above_range_set.frontier(), 4);
    ///
    /// above_range_set.add_event(6);
    /// assert_eq!(above_range_set.frontier(), 4);
    /// ```
    fn frontier(&self) -> u64 {
        self.max
    }

    /// Merges `other` `AboveRangeSet` into `self`.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut above_range_set = AboveRangeSet::new();
    /// above_range_set.add_event(1);
    /// above_range_set.add_event(3);
    /// above_range_set.add_event(4);
    /// assert_eq!(above_range_set.events(), (1, vec![3, 4]));
    ///
    /// above_range_set.join(&AboveRangeSet::from_event(3));
    /// assert_eq!(above_range_set.events(), (1, vec![3, 4]));
    ///
    /// above_range_set.join(&AboveRangeSet::from_event(5));
    /// assert_eq!(above_range_set.events(), (1, vec![3, 4, 5]));
    ///
    /// let mut other = AboveRangeSet::new();
    /// other.add_event(2);
    /// other.add_event(7);
    /// above_range_set.join(&other);
    /// assert_eq!(above_range_set.events(), (5, vec![7]));
    /// ```
    fn join(&mut self, other: &Self) {
        // the new max value is the max of both max values
        self.max = cmp::max(self.max, other.max);

        // join ranges
        self.ranges.join(&other.ranges, self.max);

        // maybe compress
        self.try_compress();
    }

    /// Returns a `AboveRangeSet` event iterator with all events from lowest to
    /// highest.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut above_range_set = AboveRangeSet::new();
    /// above_range_set.add_event(3);
    /// above_range_set.add_event(5);
    ///
    /// let mut iter = above_range_set.event_iter();
    /// assert_eq!(iter.next(), Some(3));
    /// assert_eq!(iter.next(), Some(5));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn event_iter(self) -> Self::EventIter {
        EventIter {
            current: 0,
            max: self.max,
            ranges: self.ranges.event_iter(),
        }
    }
}

impl AboveRangeSet {
    /// Tries to set a new max contiguous event.
    fn try_compress(&mut self) {
        // drop the first range while its start is right after the max
        while let Some(new_max) = self.ranges.maybe_drop_first(&self.max) {
            self.max = new_max;
        }
    }

    /// Creates a new instance from the highest contiguous event, and a sequence
    /// of extra events.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let above_range_set = AboveRangeSet::from(0, vec![2, 4, 5]);
    /// assert!(!above_range_set.is_event(1));
    /// assert!(above_range_set.is_event(2));
    /// assert!(!above_range_set.is_event(3));
    /// assert!(above_range_set.is_event(4));
    /// assert!(above_range_set.is_event(5));
    /// assert!(!above_range_set.is_event(6));
    /// ```
    pub fn from<I: IntoIterator<Item = u64>>(max: u64, iter: I) -> Self {
        let ranges = Ranges::from::<I>(iter);
        AboveRangeSet { max, ranges }
    }
}

pub struct EventIter {
    // Last contiguous value returned by the iterator
    current: u64,
    // Last contiguous value that should be returned by the iterator
    max: u64,
    // Iterator of extra ranges
    ranges: RangesIter,
}

impl Iterator for EventIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.max {
            // we've reached the last contiguous, just call next on the extra
            // ranges iterator
            self.ranges.next()
        } else {
            // compute next value
            self.current += 1;
            Some(self.current)
        }
    }
}

impl fmt::Debug for AboveRangeSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.ranges.is_empty() {
            write!(f, "{}", self.max)
        } else {
            write!(f, "({} + {:?})", self.max, self.ranges)
        }
    }
}

impl Ranges {
    /// Creates a new `Ranges` instance.
    fn new() -> Self {
        Ranges {
            ranges: BTreeMap::new(),
        }
    }

    /// Checks if there are no ranges.
    fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }

    /// Adds a new range, assuming it is new, i.e.:
    /// - none of the events within the range have already been added.
    fn add(&mut self, start: u64, mut end: u64) {
        // split map where the new range should be inserted
        let mut after_new_range = self.ranges.split_off(&start);

        let mut inserted = false;

        // check if the previous range can be extended with the new range
        if let Some(mut before) = self.ranges.last_entry() {
            let before_end = before.get_mut();
            if *before_end + 1 == start {
                // extend the previous range
                *before_end = end;

                // check if we can also extend this range with the first range
                // in the splitted off ranges
                if let Some(after) = after_new_range.first_entry() {
                    if *before_end + 1 == *after.key() {
                        // remove entry and extend range again
                        *before_end = after.remove();
                    }
                }
                // we're done, we only need to merge the splitted off ranges
                inserted = true;
            }
        }

        // if here haven't extended the previous range, then we need to create a
        // new one
        if !inserted {
            // check if we should create a new one with the provided `end`, or
            // with the end of the next range (in case they can be merged)
            if let Some(after) = after_new_range.first_entry() {
                if end + 1 == *after.key() {
                    // remove entry and extend new range to be added
                    end = after.remove();
                }
            }

            // insert new range
            self.ranges.insert(start, end);
        }

        // extend map with the ranges that have been splitted off
        self.ranges.append(&mut after_new_range);
    }

    /// Checks if the event is part of any of the ranges.
    fn contains(&self, event: &u64) -> bool {
        for (start, end) in self.ranges.iter() {
            if start <= event {
                if event <= end {
                    return true;
                }
            } else {
                // if we're in a range that starts after event, then we can give
                // up since it won't be a part of any other range that comes
                // after
                return false;
            }
        }
        return false;
    }

    /// Joins two ranges. This implementation makes no effort in being
    /// efficient.
    fn join(&mut self, other: &Self, max: u64) {
        let mut result = Ranges::new();

        // add all events from self that are higher than the new max
        for event in self.clone().event_iter() {
            if event > max {
                result.add(event, event);
            }
        }

        // add all events from `other` that are higher than the new max
        // AND haven't been added yet
        for event in other.clone().event_iter() {
            if event > max && !result.contains(&event) {
                result.add(event, event);
            }
        }

        self.ranges = result.ranges;
    }

    /// Creates a iterator for all events represented by the ranges.
    fn event_iter(self) -> RangesIter {
        RangesIter {
            current: None,
            ranges: self.ranges.into_iter(),
        }
    }

    /// Creates a new `Ranges` from a set of events.
    /// Assumes there are no repeated events.
    fn from<I: IntoIterator<Item = u64>>(iter: I) -> Self {
        let mut result = Ranges::new();
        for event in iter {
            result.add(event, event);
        }
        result
    }

    /// Drop the first range in case it can be used to update the maximum value.
    fn maybe_drop_first(&mut self, max: &u64) -> Option<u64> {
        if let Some(first_entry) = self.ranges.first_entry() {
            if max + 1 == *first_entry.key() {
                return Some(first_entry.remove());
            }
        }
        None
    }
}

pub struct RangesIter {
    current: Option<(u64, u64)>,
    ranges: std::collections::btree_map::IntoIter<u64, u64>,
}

impl Iterator for RangesIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // if currently iterating a range, then keep going
        if let Some((val, end)) = self.current {
            if val <= end {
                self.current = Some((val + 1, end));
                return Some(val);
            }
        }

        // if we haven't returned a new value from the current range, try again
        // in the next range
        self.current = self.ranges.next();
        if self.current.is_none() {
            // if there's no next range, we're done
            None
        } else {
            self.next()
        }
    }
}

impl fmt::Debug for Ranges {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.ranges)
    }
}
