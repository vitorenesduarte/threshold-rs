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
        Self::from_map(HashMap::new())
    }

    /// Creates a `Clock` from a map from actor identifier to its event set.
    pub fn from_map(clock: HashMap<A, E>) -> Self {
        Clock { clock }
    }

    /// Creates a `Clock` from a vector of tuples (actor identifier and event
    /// set).
    pub fn from_vec(clock: Vec<(A, E)>) -> Self {
        Self::from_map(clock.into_iter().collect())
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
    pub fn add_dot(&mut self, dot: &Dot<A>) {
        self.upsert(
            &dot.actor,
            |eset| eset.add_event(dot.seq),
            || (E::from_event(dot.seq), ()),
        );
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
/// use threshold::{vclock, *};
///
/// let clock = vclock::vclock_from_seqs(vec![10, 20]);
/// assert!(clock.is_element(&Dot::new(&0, 10)));
/// assert!(clock.is_element(&Dot::new(&1, 20)));
/// ```
pub fn vclock_from_seqs<I: IntoIterator<Item = u64>>(iter: I) -> VClock<u64> {
    let clock = HashMap::from_iter(
        iter.into_iter()
            .enumerate()
            .map(|(actor, seq)| (actor as u64, MaxSet::from_event(seq))),
    );
    Clock::from_map(clock)
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
