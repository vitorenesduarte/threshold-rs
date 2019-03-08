//! This module contains an implementation of a vector clock.
//!
//! The implementation is inspired in [rust-crdt's implementation](https://github.com/rust-crdt/rust-crdt/blob/master/src/vclock.rs).
//!
//! # Examples
//! ```
//! use threshold::*;
//! ```

use std::cmp;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub trait Actor: Clone + Hash + Eq + Debug {}
impl<A: Clone + Hash + Eq + Debug> Actor for A {}

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
        VClock {
            clock: HashMap::new(),
        }
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
        self.upsert(&dot.actor, dot.seq, |seq| cmp::max(seq, dot.seq));
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
}
