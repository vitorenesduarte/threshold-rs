//! This module contains an implementation of a vector clock.
//!
//! The implementation is inspired in [rust-crdt's implementation](https://github.com/rust-crdt/rust-crdt/blob/master/src/vclock.rs).
//!
//! # Examples
//! ```
//! use threshold::*;
//!
//! let actor_a = "A";
//! let mut clock_a = VClock::new();
//! let mut clock_b = VClock::new();
//!
//! clock_a.next(&actor_a);
//! let event = clock_a.next(&actor_a);
//!
//! clock_b.join(&clock_a);
//! assert!(clock_b.contains(&actor_a, event));
//! ```

use crate::*;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{self, HashMap};
use std::iter::FromIterator;

// A Vector Clock is `Clock` with `MaxSet` as `EventSet`.
pub type VClock<A> = Clock<A, MaxSet>;
// An Above Exception Clock is `Clock` with `AboveExSet` as `EventSet`.
pub type AEClock<A> = Clock<A, AboveExSet>;
// A Below Exception Clock is `Clock` with `BelowExSet` as `EventSet`.
pub type BEClock<A> = Clock<A, BelowExSet>;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Clock<A: Actor, E: EventSet> {
    /// Mapping from actor identifier to an event set
    clock: HashMap<A, E>,
}

impl<A: Actor, E: EventSet> Clock<A, E> {
    /// Returns a new `Clock` instance.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Clock {
            clock: HashMap::new(),
        }
    }

    /// Returns a new `Clock` mapping each actor to a bottom entry.
    ///
    /// # Examples
    /// ```
    /// use std::collections::HashMap;
    /// use std::iter::FromIterator;
    /// use threshold::*;
    ///
    /// let actors = vec!["A", "B"];
    /// let vclock = VClock::with(actors);
    /// assert_eq!(
    ///     vclock.frontier(),
    ///     HashMap::from_iter(vec![(&"A", 0), (&"B", 0)]),
    /// );
    /// ```
    pub fn with<I: IntoIterator<Item = A>>(iter: I) -> Self {
        Clock {
            clock: iter.into_iter().map(|actor| (actor, E::new())).collect(),
        }
    }

    /// Creates a `Clock` from an iterator of tuples (actor identifier and event
    /// set).
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let a = ("A", MaxSet::from_event(10));
    /// let b = ("B", MaxSet::from_event(20));
    /// let vclock = Clock::from(vec![a, b]);
    ///
    /// assert!(vclock.contains(&"A", 9));
    /// assert!(!vclock.contains(&"A", 11));
    /// ```
    pub fn from<I: IntoIterator<Item = (A, E)>>(iter: I) -> Self {
        Clock {
            clock: HashMap::from_iter(iter),
        }
    }

    /// Returns the number of actors in the clock.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let a = ("A", MaxSet::from_event(10));
    /// let b = ("B", MaxSet::from_event(20));
    /// let vclock = Clock::from(vec![a, b]);
    ///
    /// assert_eq!(vclock.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.clock.len()
    }

    /// Checks that a clock is empty.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let a = ("A", MaxSet::from_event(10));
    /// let b = ("B", MaxSet::from_event(20));
    /// let mut vclock = Clock::from(vec![a, b]);
    ///
    /// assert!(!vclock.is_empty());
    ///
    /// vclock = VClock::new();
    /// assert!(vclock.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.clock.is_empty()
    }

    /// Returns the next event for the `actor` while updating its entry in the
    /// clock.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let actor_a = "A";
    /// let actor_b = "B";
    ///
    /// let mut clock = VClock::new();
    /// let next = clock.next(&actor_a);
    /// assert_eq!(next, 1);
    ///
    /// let next = clock.next(&actor_a);
    /// assert_eq!(next, 2);
    ///
    /// let next = clock.next(&actor_a);
    /// assert_eq!(next, 3);
    /// ```
    pub fn next(&mut self, actor: &A) -> u64 {
        self.upsert(actor, |eset| eset.next_event(), || (E::from_event(1), 1))
    }

    /// If the actor is in already the clock, its entry is updated using
    /// function `map`. Otherwise, the output of `default` is inserted.
    fn upsert<F, D, R>(&mut self, actor: &A, mut map: F, default: D) -> R
    where
        F: FnMut(&mut E) -> R,
        D: FnOnce() -> (E, R),
    {
        match self.clock.get_mut(actor) {
            Some(eset) => map(eset),
            None => {
                let (value, result) = default();
                self.clock.insert(actor.clone(), value);
                result
            }
        }
    }

    /// Adds an event to the clock.
    /// If the clock did not have this event present, `true` is returned.
    /// If the clock did have this event present, `false` is returned.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let actor_a = "A";
    /// let actor_b = "B";
    ///
    /// let mut clock = VClock::new();
    ///
    /// assert!(!clock.contains(&actor_a, 1));
    /// clock.add(&actor_a, 1);
    /// assert!(clock.contains(&actor_a, 1));
    ///
    /// assert!(!clock.contains(&actor_b, 1));
    /// clock.add(&actor_b, 1);
    /// assert!(clock.contains(&actor_b, 1));
    /// ```
    pub fn add(&mut self, actor: &A, seq: u64) -> bool {
        self.upsert(
            actor,
            |eset| eset.add_event(seq),
            || (E::from_event(seq), true),
        )
    }

    /// Adds a range of events to the clock.
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let actor_a = "A";
    /// let actor_b = "B";
    ///
    /// let mut clock_a = VClock::new();
    /// clock_a.add_range(&actor_a, 10, 20);
    /// assert!(clock_a.contains(&actor_a, 10));
    /// assert!(clock_a.contains(&actor_a, 11));
    /// assert!(!clock_a.contains(&actor_a, 21));
    /// ```
    pub fn add_range(&mut self, actor: &A, start: u64, end: u64) -> bool {
        self.upsert(
            actor,
            |eset| eset.add_event_range(start, end),
            || (E::from_event_range(start, end), true),
        )
    }

    /// Checks if an event is part of the clock.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let actor_a = "A";
    ///
    /// let mut clock = VClock::new();
    /// assert!(!clock.contains(&actor_a, 1));
    /// clock.add(&actor_a, 1);
    /// assert!(clock.contains(&actor_a, 1));
    /// assert!(!clock.contains(&actor_a, 2));
    ///
    /// clock.add(&actor_a, 3);
    /// assert!(clock.contains(&actor_a, 1));
    /// assert!(clock.contains(&actor_a, 2));
    /// assert!(clock.contains(&actor_a, 3));
    /// ```
    pub fn contains(&self, actor: &A, seq: u64) -> bool {
        self.clock
            .get(actor)
            .map_or(false, |eset| eset.is_event(seq))
    }

    /// Returns the clock frontier.
    ///
    /// # Examples
    /// ```
    /// use std::collections::HashMap;
    /// use std::iter::FromIterator;
    /// use threshold::*;
    ///
    /// let a = ("A", AboveExSet::from_events(vec![1, 2, 4]));
    /// let b = ("B", AboveExSet::from_events(vec![1, 2, 3, 5, 6]));
    /// let clock = Clock::from(vec![a, b]);
    ///
    /// assert_eq!(
    ///     clock.frontier(),
    ///     HashMap::from_iter(vec![(&"A", 2), (&"B", 3)])
    /// );
    /// ```
    pub fn frontier(&self) -> HashMap<&A, u64> {
        self.clock
            .iter()
            .map(|(actor, eset)| (actor, eset.frontier()))
            .collect()
    }

    /// By looking at this `Clock`'s frontier, it computes the event that's been
    /// generated in at least `threshold` actors.
    ///
    /// # Examples
    /// ```
    /// use threshold::{clock, *};
    ///
    /// let aset = AboveExSet::from_events(vec![1, 2, 4]);
    /// let bset = AboveExSet::from_events(vec![1, 2, 3, 5]);
    /// let clock = Clock::from(vec![("A", aset), ("B", bset)]);
    /// assert_eq!(clock.frontier_threshold(1), Some(3));
    /// assert_eq!(clock.frontier_threshold(2), Some(2));
    /// assert_eq!(clock.frontier_threshold(3), None);
    ///
    /// let aset = AboveExSet::from_events(vec![1, 2, 3, 5]);
    /// let bset = AboveExSet::from_events(vec![1, 2, 3, 5]);
    /// let clock = Clock::from(vec![("A", aset), ("B", bset)]);
    /// assert_eq!(clock.frontier_threshold(1), Some(3));
    /// assert_eq!(clock.frontier_threshold(2), Some(3));
    ///
    /// let clock = clock::vclock_from_seqs(vec![2, 1, 3]);
    /// assert_eq!(clock.frontier_threshold(1), Some(3));
    /// assert_eq!(clock.frontier_threshold(2), Some(2));
    /// assert_eq!(clock.frontier_threshold(3), Some(1));
    ///
    /// let clock = clock::vclock_from_seqs(vec![4, 4, 5, 3, 2]);
    /// assert_eq!(clock.frontier_threshold(1), Some(5));
    /// assert_eq!(clock.frontier_threshold(2), Some(4));
    /// assert_eq!(clock.frontier_threshold(3), Some(4));
    /// assert_eq!(clock.frontier_threshold(4), Some(3));
    /// assert_eq!(clock.frontier_threshold(5), Some(2));
    /// assert_eq!(clock.frontier_threshold(6), None);
    /// ```
    pub fn frontier_threshold(&self, threshold: usize) -> Option<u64> {
        assert!(threshold > 0);
        let clock_size = self.clock.len();
        if threshold <= clock_size {
            // get frontiers and sort them
            let mut frontiers: Vec<_> =
                self.clock.iter().map(|(_, eset)| eset.frontier()).collect();
            frontiers.sort_unstable();

            // get the frontier at the correct threshold
            frontiers.into_iter().nth(clock_size - threshold)
        } else {
            None
        }
    }

    /// Merges vector clock `other` passed as argument into `self`.
    /// After merge, all events in `other` are events in `self`.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let actor_a = "A";
    /// let mut clock_a = VClock::new();
    /// let mut clock_b = VClock::new();
    ///
    /// clock_a.next(&actor_a);
    /// let event = clock_a.next(&actor_a);
    ///
    /// clock_b.join(&clock_a);
    /// assert!(clock_b.contains(&actor_a, event));
    /// ```
    pub fn join(&mut self, other: &Self) {
        for (actor, eset) in other.clock.iter() {
            self.upsert(
                actor,
                |current_eset| current_eset.join(eset),
                || (eset.clone(), ()),
            );
        }
    }

    /// Subtracts an event `subtract` to the events generated by some `actor`.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let actor_a = "A";
    ///
    /// let mut clock = VClock::new();
    /// clock.add(&actor_a, 1);
    /// clock.add(&actor_a, 2);
    /// clock.add(&actor_a, 3);
    ///
    /// let subtract = MaxSet::from_event(0);
    /// let subtracted: Vec<_> = clock.subtract_iter(&actor_a, subtract).collect();
    /// assert_eq!(subtracted, vec![1, 2, 3]);
    ///
    /// let subtract = MaxSet::from_event(1);
    /// let subtracted: Vec<_> = clock.subtract_iter(&actor_a, subtract).collect();
    /// assert_eq!(subtracted, vec![2, 3]);
    ///
    /// let subtract = MaxSet::from_event(2);
    /// let subtracted: Vec<_> = clock.subtract_iter(&actor_a, subtract).collect();
    /// assert_eq!(subtracted, vec![3]);
    ///
    /// let subtract = MaxSet::from_event(3);
    /// let subtracted: Vec<_> = clock.subtract_iter(&actor_a, subtract).collect();
    /// assert_eq!(subtracted, vec![]);
    /// ```
    pub fn subtract_iter<S: EventSet>(
        &self,
        actor: &A,
        subtract: S,
    ) -> impl Iterator<Item = u64> {
        let eset = match self.clock.get(actor) {
            Some(eset) => eset.clone(),
            None => E::new(),
        };
        crate::subtract_iter(eset, subtract)
    }
}

/// Creates a new vector clock from a list of sequences.
/// `u64` are used as actor identifers and:
/// - the first sequence is mapped to actor number 0
/// - the last sequence is mapped to actor number #sequences - 1
///
/// # Examples
/// ```
/// use threshold::{clock, *};
///
/// let clock = clock::vclock_from_seqs(vec![10, 20]);
/// assert!(clock.contains(&0, 10));
/// assert!(clock.contains(&1, 20));
/// ```
pub fn vclock_from_seqs<I: IntoIterator<Item = u64>>(iter: I) -> VClock<u64> {
    Clock::from(
        iter.into_iter()
            .enumerate()
            .map(|(actor, seq)| (actor as u64, MaxSet::from_event(seq))),
    )
}

pub struct IntoIter<A: Actor, E: EventSet>(hash_map::IntoIter<A, E>);

impl<A: Actor, E: EventSet> Iterator for IntoIter<A, E> {
    type Item = (A, E);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<A: Actor, E: EventSet> IntoIterator for Clock<A, E> {
    type Item = (A, E);
    type IntoIter = IntoIter<A, E>;

    /// Returns a `Clock` into-iterator.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut clock = VClock::new();
    /// clock.next(&"A");
    /// clock.next(&"A");
    /// clock.next(&"B");
    ///
    /// for (actor, eset) in clock {
    ///     match actor {
    ///         "A" => assert_eq!(eset, MaxSet::from_event(2)),
    ///         "B" => assert_eq!(eset, MaxSet::from_event(1)),
    ///         _ => panic!("unexpected actor name"),
    ///     }
    /// }
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.clock.into_iter())
    }
}
