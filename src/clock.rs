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
//! clock_a.next_dot(&actor_a);
//! let dot_a2 = clock_a.next_dot(&actor_a);
//!
//! clock_b.join(&clock_a);
//! assert!(clock_b.is_element(&dot_a2));
//! ```

use crate::*;
use std::collections::hash_map::{self, HashMap};
use std::iter::FromIterator;

// A Vector Clock is `Clock` with `MaxSet` as `EventSet`.
pub type VClock<A> = Clock<A, MaxSet>;
// An Above Exception Clock is `Clock` with `AboveExSet` as `EventSet`.
pub type AEClock<A> = Clock<A, AboveExSet>;
// A Below Exception Clock is `Clock` with `BelowExSet` as `EventSet`.
pub type BEClock<A> = Clock<A, BelowExSet>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dot<A: Actor> {
    /// Actor identifer
    actor: A,
    /// Sequence number
    seq: u64,
}

impl<A: Actor> Dot<A> {
    /// Returns a new `Dot` instance.
    pub fn new(actor: &A, seq: u64) -> Self {
        Dot {
            actor: actor.clone(),
            seq,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clock<A: Actor, E: EventSet> {
    /// Mapping from actor identifier to an event set
    clock: HashMap<A, E>,
}

impl<A: Actor, E: EventSet> Clock<A, E> {
    /// Returns a new `Clock` instance.
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
    /// assert!(vclock.is_element(&Dot::new(&"A", 9)));
    /// assert!(!vclock.is_element(&Dot::new(&"A", 11)));
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
    /// assert_eq!(vclock.actor_count(), 2);
    /// ```
    pub fn actor_count(&self) -> usize {
        self.clock.len()
    }

    /// Returns a new `Dot` for the `actor` while updating the clock.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let actor_a = "A";
    /// let actor_b = "B";
    ///
    /// let mut clock = VClock::new();
    /// let dot_a1 = clock.next_dot(&actor_a);
    /// assert_eq!(Dot::new(&actor_a, 1), dot_a1);
    ///
    /// let dot_a2 = clock.next_dot(&actor_a);
    /// assert_eq!(Dot::new(&actor_a, 2), dot_a2);
    ///
    /// let dot_b1 = clock.next_dot(&actor_b);
    /// assert_eq!(Dot::new(&actor_b, 1), dot_b1);
    /// ```
    pub fn next_dot(&mut self, actor: &A) -> Dot<A> {
        let seq = self.upsert(
            actor,
            |eset| eset.next_event(),
            || (E::from_event(1), 1),
        );
        Dot::new(actor, seq)
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

    /// Adds a `Dot` to the clock.
    /// If the clock did not have this `Dot` present, `true` is returned.
    /// If the clock did have this `Dot` present, `false` is returned.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let actor_a = "A";
    /// let actor_b = "B";
    ///
    /// let mut clock_a = VClock::new();
    /// let mut clock_b = VClock::new();
    ///
    /// let dot_a1 = clock_a.next_dot(&actor_a);
    ///
    /// assert!(!clock_b.is_element(&dot_a1));
    /// clock_b.add_dot(&dot_a1);
    /// assert!(clock_b.is_element(&dot_a1));
    /// ```
    pub fn add_dot(&mut self, dot: &Dot<A>) -> bool {
        self.add(&dot.actor, dot.seq)
    }

    /// Similar to `add_dot` but does not require a `Dot` instance.
    pub fn add(&mut self, actor: &A, seq: u64) -> bool {
        self.upsert(
            actor,
            |eset| eset.add_event(seq),
            || (E::from_event(seq), true),
        )
    }

    /// Checks if an `Dot` is part of the clock.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let actor_a = "A";
    ///
    /// let dot_a1 = Dot::new(&actor_a, 1);
    /// let dot_a2 = Dot::new(&actor_a, 2);
    /// let dot_a3 = Dot::new(&actor_a, 3);
    ///
    /// let mut clock = VClock::new();
    /// assert!(!clock.is_element(&dot_a1));
    /// clock.add_dot(&dot_a1);
    /// assert!(clock.is_element(&dot_a1));
    /// assert!(!clock.is_element(&dot_a2));
    ///
    /// clock.add_dot(&dot_a3);
    /// assert!(clock.is_element(&dot_a1));
    /// assert!(clock.is_element(&dot_a2));
    /// assert!(clock.is_element(&dot_a3));
    /// ```
    pub fn is_element(&self, dot: &Dot<A>) -> bool {
        self.clock
            .get(&dot.actor)
            .map_or(false, |eset| eset.is_event(&dot.seq))
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
    /// assert_eq!(clock.frontier_threshold(1), Some(2));
    /// assert_eq!(clock.frontier_threshold(2), Some(3));
    /// assert_eq!(clock.frontier_threshold(3), None);
    ///
    /// let aset = AboveExSet::from_events(vec![1, 2, 3, 5]);
    /// let bset = AboveExSet::from_events(vec![1, 2, 3, 5]);
    /// let clock = Clock::from(vec![("A", aset), ("B", bset)]);
    /// assert_eq!(clock.frontier_threshold(1), Some(3));
    /// assert_eq!(clock.frontier_threshold(2), Some(3));
    ///
    /// let clock = clock::vclock_from_seqs(vec![2, 1, 3]);
    /// assert_eq!(clock.frontier_threshold(1), Some(1));
    /// assert_eq!(clock.frontier_threshold(2), Some(2));
    /// assert_eq!(clock.frontier_threshold(3), Some(3));
    ///
    /// let clock = clock::vclock_from_seqs(vec![4, 4, 5, 3, 2]);
    /// assert_eq!(clock.frontier_threshold(1), Some(2));
    /// assert_eq!(clock.frontier_threshold(2), Some(3));
    /// assert_eq!(clock.frontier_threshold(3), Some(4));
    /// assert_eq!(clock.frontier_threshold(4), Some(4));
    /// assert_eq!(clock.frontier_threshold(5), Some(5));
    /// assert_eq!(clock.frontier_threshold(6), None);
    /// ```
    pub fn frontier_threshold(&self, threshold: usize) -> Option<u64> {
        assert!(threshold > 0);
        let mut frontiers: Vec<_> =
            self.clock.iter().map(|(_, eset)| eset.frontier()).collect();
        frontiers.sort_unstable();
        frontiers.into_iter().nth(threshold - 1)
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
    /// clock_a.next_dot(&actor_a);
    /// let dot_a2 = clock_a.next_dot(&actor_a);
    ///
    /// clock_b.join(&clock_a);
    /// assert!(clock_b.is_element(&dot_a2));
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
/// assert!(clock.is_element(&Dot::new(&0, 10)));
/// assert!(clock.is_element(&Dot::new(&1, 20)));
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
    /// clock.next_dot(&"A");
    /// clock.next_dot(&"A");
    /// clock.next_dot(&"B");
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
