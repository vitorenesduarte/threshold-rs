//! This module contains an implementation of a vector clock.
//!
//! The implementation is inspired in [rust-crdt's implementation](https://github.com/rust-crdt/rust-crdt/blob/master/src/vclock.rs).
//!
//! # Examples
//! ```
//! use threshold::*;
//!
//! let actor_a = "A";
//! let mut vclock_a = VClock::new();
//! let mut vclock_b = VClock::new();
//!
//! vclock_a.next_dot(&actor_a);
//! let dot_a2 = vclock_a.next_dot(&actor_a);
//!
//! vclock_b.join(&vclock_a);
//! assert!(vclock_b.is_element(&dot_a2));
//! ```

use crate::traits::Actor;
use std::collections::hash_map::{self, HashMap};
use std::iter::FromIterator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dot<T: Actor> {
    /// Actor identifer
    actor: T,
    /// Sequence number
    seq: u64,
}

impl<T: Actor> Dot<T> {
    /// Returns a new `Dot` instance.
    pub fn new(actor: &T, seq: u64) -> Self {
        Dot {
            actor: actor.clone(),
            seq,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VClock<T: Actor> {
    /// Mapping from actor to its last event sequence
    clock: HashMap<T, u64>,
}

impl<T: Actor> VClock<T> {
    /// Returns a new `VClock` instance.
    pub fn new() -> Self {
        Self::from_map(HashMap::new())
    }

    /// Creates a `VClock` from a map from actor identifier to its sequence
    /// number.
    pub fn from_map(clock: HashMap<T, u64>) -> Self {
        VClock { clock }
    }

    /// Creates a `VClock` from a vector of tuples (actor identifier and
    /// sequence number).
    pub fn from_vec(clock: Vec<(T, u64)>) -> Self {
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
    /// let mut vclock = VClock::new();
    /// let dot_a1 = vclock.next_dot(&actor_a);
    /// assert_eq!(Dot::new(&actor_a, 1), dot_a1);
    ///
    /// let dot_a2 = vclock.next_dot(&actor_a);
    /// assert_eq!(Dot::new(&actor_a, 2), dot_a2);
    ///
    /// let dot_b1 = vclock.next_dot(&actor_b);
    /// assert_eq!(Dot::new(&actor_b, 1), dot_b1);
    /// ```
    pub fn next_dot(&mut self, actor: &T) -> Dot<T> {
        let seq = self.upsert(actor, 1, |seq| seq + 1);
        Dot::new(actor, seq)
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
    /// let mut vclock_a = VClock::new();
    /// let mut vclock_b = VClock::new();
    ///
    /// let dot_a1 = vclock_a.next_dot(&actor_a);
    ///
    /// assert!(!vclock_b.is_element(&dot_a1));
    /// vclock_b.add_dot(&dot_a1);
    /// assert!(vclock_b.is_element(&dot_a1));
    /// ```
    pub fn add_dot(&mut self, dot: &Dot<T>) {
        self.add_entry(&dot.actor, dot.seq);
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
    /// let mut vclock = VClock::new();
    /// assert!(!vclock.is_element(&dot_a1));
    /// vclock.add_dot(&dot_a1);
    /// assert!(vclock.is_element(&dot_a1));
    /// assert!(!vclock.is_element(&dot_a2));
    ///
    /// vclock.add_dot(&dot_a3);
    /// assert!(vclock.is_element(&dot_a1));
    /// assert!(vclock.is_element(&dot_a2));
    /// assert!(vclock.is_element(&dot_a3));
    /// ```
    pub fn is_element(&self, dot: &Dot<T>) -> bool {
        self.clock
            .get(&dot.actor)
            .map_or(false, |&seq| dot.seq <= seq)
    }

    /// Merges vector clock `other` passed as argument into `self`.
    /// After merge, all events in `other` are events in `self`.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let actor_a = "A";
    /// let mut vclock_a = VClock::new();
    /// let mut vclock_b = VClock::new();
    ///
    /// vclock_a.next_dot(&actor_a);
    /// let dot_a2 = vclock_a.next_dot(&actor_a);
    ///
    /// vclock_b.join(&vclock_a);
    /// assert!(vclock_b.is_element(&dot_a2));
    /// ```
    pub fn join(&mut self, other: &Self) {
        for (actor, &seq) in other.clock.iter() {
            self.add_entry(actor, seq);
        }
    }

    /// Update a single actor entry.
    fn add_entry(&mut self, actor: &T, seq: u64) {
        self.upsert(actor, seq, |current_seq| std::cmp::max(current_seq, seq));
    }

    /// If the actor is in already the clock, its entry is updated using
    /// function `map`. Otherwise, a `default` value is inserted.
    fn upsert<F>(&mut self, actor: &T, default: u64, map: F) -> u64
    where
        F: FnOnce(u64) -> u64,
    {
        match self.clock.get_mut(actor) {
            Some(seq) => {
                *seq = map(*seq);
                *seq
            }
            None => {
                self.clock.insert(actor.clone(), default);
                default
            }
        }
    }
}

/// Creates a new `VClock` from a list of sequences.
/// `u64` are used as actor identifers and:
/// - the first sequence is mapped to actor number 0
/// - the last sequence is mapped to actor number #sequences - 1
///
/// # Examples
/// ```
/// use threshold::{vclock, *};
///
/// let vclock = vclock::from_seqs(vec![10, 20]);
/// assert!(vclock.is_element(&Dot::new(&0, 10)));
/// assert!(vclock.is_element(&Dot::new(&1, 20)));
/// ```
pub fn from_seqs<I: IntoIterator<Item = u64>>(iter: I) -> VClock<u64> {
    let clock = HashMap::from_iter(
        iter.into_iter()
            .enumerate()
            .map(|(actor, seq)| (actor as u64, seq)),
    );
    VClock::from_map(clock)
}

pub struct IntoIter<T: Actor>(hash_map::IntoIter<T, u64>);

impl<T: Actor> Iterator for IntoIter<T> {
    type Item = (T, u64);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<T: Actor> IntoIterator for VClock<T> {
    type Item = (T, u64);
    type IntoIter = IntoIter<T>;

    /// Returns a `VClock` into iterator.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut vclock = VClock::new();
    /// vclock.next_dot(&"A");
    /// vclock.next_dot(&"A");
    /// vclock.next_dot(&"B");
    ///
    /// for (actor, seq) in vclock {
    ///     match actor {
    ///         "A" => assert_eq!(seq, 2),
    ///         "B" => assert_eq!(seq, 1),
    ///         _ => panic!("unexpected actor name"),
    ///     }
    /// }
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.clock.into_iter())
    }
}
