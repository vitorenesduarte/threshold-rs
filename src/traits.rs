use std::fmt::Debug;
use std::hash::Hash;

/// Count trait to be used in `MultiSet`.
pub trait Count: Copy {
    /// Return a zero count.
    fn zero() -> Self;

    /// Add to the count.
    fn add(&mut self, other: Self);
}

impl Count for u64 {
    /// Return a zero count.
    fn zero() -> Self {
        0
    }

    /// Add to the count.
    fn add(&mut self, other: Self) {
        *self += other;
    }
}

impl Count for (u64, u64) {
    /// Return a zero count.
    fn zero() -> Self {
        (0, 0)
    }

    /// Add to the count.
    fn add(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
    }
}

/// Actor trait to be used in `Clock`'s or `TClock`'s.
pub trait Actor: Debug + Clone + Hash + Eq + Ord {}
impl<A: Debug + Clone + Hash + Eq + Ord> Actor for A {}

/// EventSet trait to be implemented by `MaxSet`, `BelowExSet` and `AboveExSet`.
pub trait EventSet: Clone + Debug + Default {
    type EventIter: Iterator<Item = u64>;
    /// Returns a new instance.
    fn new() -> Self;

    /// Creates a new instance from `event`.
    #[inline]
    fn from_event(event: u64) -> Self {
        let mut eset = Self::new();
        eset.add_event(event);
        eset
    }

    /// Creates a new instance from a range of events.
    #[inline]
    fn from_event_range(start: u64, end: u64) -> Self {
        let mut eset = Self::new();
        eset.add_event_range(start, end);
        eset
    }

    /// Creates a new instance from several `events`.
    #[inline]
    fn from_events<I: IntoIterator<Item = u64>>(iter: I) -> Self {
        let mut eset = Self::new();
        for event in iter {
            eset.add_event(event);
        }
        eset
    }

    /// Generates the next event.
    fn next_event(&mut self) -> u64;

    /// Adds an event to the set.
    fn add_event(&mut self, event: u64) -> bool;

    /// Adds a range of events to the set.
    fn add_event_range(&mut self, start: u64, end: u64) -> bool {
        let mut res = false;
        (start..=end).for_each(|event| {
            let added = self.add_event(event);
            res = res || added;
        });
        res
    }

    /// Checks if an event is part of the set.
    fn is_event(&self, event: u64) -> bool;

    /// Returns all events seen as a pair.
    ///
    /// For `MaxSet`:
    /// - the first component is the highest event
    /// - the second component is empty
    ///
    /// For `BelowExSet`:
    /// - the first component is the highest event
    /// - the second component is a set of exceptions
    ///
    /// For `AboveExSet`:
    /// - the first component is the highest event in a contiguous sequence
    /// - the second component is a set of outstanding events
    ///
    /// If we've seen events [1, 2, 3, 5, 6], this function returns in
    /// - `MaxSet`: (6, [])
    /// - `BelowExSet`: (6, \[4\])
    /// - `AboveExSet`: (3, \[5, 6\])
    fn events(&self) -> (u64, Vec<u64>);

    /// Returns the frontier (the highest contiguous event seen).
    fn frontier(&self) -> u64;

    /// Merges `other` `EventSet` into `self`.
    fn join(&mut self, other: &Self);

    /// Returns an iterator containing all elements represented by this event
    /// set.
    fn event_iter(self) -> Self::EventIter;
}

pub fn subtract_iter<E, S>(from: E, subtract: S) -> SubtractIter<E, S>
where
    E: EventSet,
    S: EventSet,
{
    SubtractIter {
        event_iter: from.event_iter(),
        subtract,
    }
}

pub struct SubtractIter<E: EventSet, S> {
    event_iter: E::EventIter,
    subtract: S,
}

impl<E, S> Iterator for SubtractIter<E, S>
where
    E: EventSet,
    S: EventSet,
{
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        match self.event_iter.next() {
            Some(event) => {
                if self.subtract.is_event(event) {
                    self.next()
                } else {
                    Some(event)
                }
            }
            None => None,
        }
    }
}
