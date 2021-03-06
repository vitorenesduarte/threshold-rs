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
use std::fmt;
use std::iter::FromIterator;

// A Vector Clock is `Clock` with `MaxSet` as `EventSet`.
pub type VClock<A> = Clock<A, MaxSet>;
// An Above Exception Clock is `Clock` with `AboveExSet` as `EventSet`.
pub type AEClock<A> = Clock<A, AboveExSet>;
// An Above Range Clock is `Clock` with `AboveRangeSet` as `EventSet`.
pub type ARClock<A> = Clock<A, AboveRangeSet>;
// A Below Exception Clock is `Clock` with `BelowExSet` as `EventSet`.
pub type BEClock<A> = Clock<A, BelowExSet>;

#[derive(Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
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
    ///     VClock::from(vec![("A", MaxSet::from(0)), ("B", MaxSet::from(0))])
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

    /// Retrieves the event set associated with some `actor`.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let actor_a = "A";
    ///
    /// let mut clock = VClock::new();
    /// assert_eq!(clock.get(&actor_a), None);
    ///
    /// clock.add(&actor_a, 1);
    /// clock.add(&actor_a, 2);
    /// let max_set = clock.get(&actor_a).expect("there should be an event set");
    /// let mut iter = max_set.clone().event_iter();
    ///
    /// assert_eq!(iter.next(), Some(1));
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn get(&self, actor: &A) -> Option<&E> {
        self.clock.get(actor)
    }

    /// Retrieves (a mutable reference to) the event set associated with some
    /// `actor`.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let actor_a = "A";
    ///
    /// let mut clock = VClock::new();
    /// assert_eq!(clock.get_mut(&actor_a), None);
    ///
    /// clock.add(&actor_a, 1);
    /// clock.add(&actor_a, 2);
    /// let max_set = clock
    ///     .get_mut(&actor_a)
    ///     .expect("there should be an event set");
    /// max_set.add_event(3);
    /// let mut iter = max_set.clone().event_iter();
    ///
    /// assert_eq!(iter.next(), Some(1));
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), Some(3));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn get_mut(&mut self, actor: &A) -> Option<&mut E> {
        self.clock.get_mut(actor)
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
    ///     VClock::from(vec![("A", MaxSet::from(2)), ("B", MaxSet::from(3))])
    /// );
    /// ```
    pub fn frontier(&self) -> VClock<A> {
        let frontier = self.clock.iter().map(|(actor, eset)| {
            (actor.clone(), MaxSet::from(eset.frontier()))
        });
        VClock::from(frontier)
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
        debug_assert!(threshold > 0);
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

    /// Merges clock `other` passed as argument into `self`.
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

    /// Intersects clock `other` passed as argument with `self`.
    /// After intersection, only the common events are in `self`.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let actor_a = "A";
    /// let mut clock_a = VClock::new();
    /// let mut clock_b = VClock::new();
    ///
    /// let event = clock_a.next(&actor_a);
    ///
    /// clock_b.meet(&clock_a);
    /// assert!(!clock_b.contains(&actor_a, event));
    ///
    /// clock_b.next(&actor_a);
    /// clock_b.meet(&clock_a);
    /// assert!(clock_b.contains(&actor_a, event));
    /// ```
    pub fn meet(&mut self, other: &Self) {
        let mut to_remove = Vec::new();
        for (actor, eset) in self.clock.iter_mut() {
            if let Some(other_eset) = other.get(actor) {
                eset.meet(other_eset);
            } else {
                to_remove.push(actor.clone());
            }
        }

        // at this point, `to_remove` contains the set of actors are present in
        // the local clock but not in the remote clock
        // - these actors shouldn't be in the final clock, so let's remove them
        for actor in to_remove {
            self.clock.remove(&actor);
        }
    }

    /// Returns a `Clock` iterator.
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
    /// for (&actor, eset) in clock.iter() {
    ///     match actor {
    ///         "A" => assert_eq!(eset, &MaxSet::from_event(2)),
    ///         "B" => assert_eq!(eset, &MaxSet::from_event(1)),
    ///         _ => panic!("unexpected actor name"),
    ///     }
    /// }
    /// ```
    pub fn iter<'a>(&self) -> Iter<'_, A, E> {
        Iter(self.clock.iter())
    }

    /// Returns a `Clock` mutable iterator.
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
    /// for (&actor, eset) in clock.iter_mut() {
    ///     if actor == "A" {
    ///         eset.add_event(3);
    ///     }
    /// }
    ///
    /// let max_set = clock.get(&"A").expect("there should be an event set");
    /// assert_eq!(max_set, &MaxSet::from_event(3));
    /// ```
    pub fn iter_mut<'a>(&mut self) -> IterMut<'_, A, E> {
        IterMut(self.clock.iter_mut())
    }

    pub fn subtracted(&self, other: &Self) -> HashMap<A, Vec<u64>> {
        self.clock
            .iter()
            .map(|(actor, eset)| {
                let subtracted = if let Some(other_eset) = other.get(actor) {
                    eset.subtracted(other_eset)
                } else {
                    eset.clone().event_iter().collect()
                };
                (actor.clone(), subtracted)
            })
            .collect()
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

pub struct Iter<'a, A: Actor, E: EventSet>(hash_map::Iter<'a, A, E>);

impl<'a, A: Actor, E: EventSet> Iterator for Iter<'a, A, E> {
    type Item = (&'a A, &'a E);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub struct IterMut<'a, A: Actor, E: EventSet>(hash_map::IterMut<'a, A, E>);

impl<'a, A: Actor, E: EventSet> Iterator for IterMut<'a, A, E> {
    type Item = (&'a A, &'a mut E);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<A: Actor, E: EventSet> fmt::Debug for Clock<A, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let clock: std::collections::BTreeMap<_, _> =
            self.clock.iter().collect();
        write!(f, "{:?}", clock)
    }
}
