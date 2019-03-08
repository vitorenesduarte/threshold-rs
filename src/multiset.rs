//! This module contains an implementation of a multi-set supporting a threshold
//! operation.
//!
//! The concept of threshold is explained in detail in [this blog post](https://vitorenes.org/post/2018/11/threshold-union/).
//!
//! # Examples
//! ```
//! use std::collections::HashSet;
//! use std::iter::FromIterator;
//! use threshold::*;
//!
//! let mut mset = MultiSet::new();
//!
//! mset.add(vec!["a", "b"]);
//! assert_eq!(mset.threshold(1), HashSet::from_iter(vec![&"a", &"b"]));
//! assert_eq!(mset.threshold(2), HashSet::new());
//!
//! mset.add(vec!["a", "c"]);
//! assert_eq!(
//!     mset.threshold(1),
//!     HashSet::from_iter(vec![&"a", &"b", &"c"])
//! );
//! assert_eq!(mset.threshold(2), HashSet::from_iter(vec![&"a"]));
//! ```

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub struct MultiSet<T: Hash + Eq> {
    /// Number of occurrences of each element added
    occurrences: HashMap<T, u64>,
}

impl<T: Hash + Eq> MultiSet<T> {
    /// Returns a new `MultiSet` instance.
    pub fn new() -> Self {
        MultiSet {
            occurrences: HashMap::new(),
        }
    }

    /// Adds several elements to the `MultiSet`.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut mset = MultiSet::new();
    /// assert_eq!(mset.count(&"a"), 0);
    ///
    /// mset.add(vec!["a", "b"]);
    /// assert_eq!(mset.count(&"a"), 1);
    /// assert_eq!(mset.count(&"b"), 1);
    /// ```
    pub fn add<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for elem in iter {
            self.add_elem(elem);
        }
    }

    /// Adds a single element to the `MultiSet`.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut mset = MultiSet::new();
    /// assert_eq!(mset.count(&"a"), 0);
    ///
    /// mset.add_elem("a");
    /// assert_eq!(mset.count(&"a"), 1);
    /// ```
    pub fn add_elem(&mut self, elem: T) {
        match self.occurrences.get_mut(&elem) {
            Some(count) => {
                *count += 1;
            }
            None => {
                self.occurrences.insert(elem, 1);
            }
        }
    }

    /// Returns the number of occurrences of an element.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut mset = MultiSet::new();
    /// assert_eq!(mset.count(&"a"), 0);
    ///
    /// mset.add(vec!["a", "b"]);
    /// assert_eq!(mset.count(&"a"), 1);
    /// assert_eq!(mset.count(&"b"), 1);
    /// assert_eq!(mset.count(&"c"), 0);
    ///
    /// mset.add(vec!["a", "c"]);
    /// assert_eq!(mset.count(&"a"), 2);
    /// assert_eq!(mset.count(&"b"), 1);
    /// assert_eq!(mset.count(&"c"), 1);
    /// assert_eq!(mset.count(&"d"), 0);
    /// ```
    pub fn count(&self, elem: &T) -> u64 {
        self.occurrences.get(elem).map_or(0, |&count| count)
    }

    /// Returns the elements in the `MultiSet` such that its multiplicity is bigger or equal than a given [threshold](https://vitorenes.org/post/2018/11/threshold-union/).
    ///
    /// # Examples
    /// ```
    /// use std::collections::HashSet;
    /// use std::iter::FromIterator;
    /// use threshold::*;
    ///
    /// let mut mset = MultiSet::new();
    /// assert_eq!(mset.threshold(1), HashSet::new());
    ///
    /// mset.add(vec!["a", "b"]);
    /// assert_eq!(mset.threshold(1), HashSet::from_iter(vec![&"a", &"b"]));
    /// assert_eq!(mset.threshold(2), HashSet::new());
    ///
    /// mset.add(vec!["a", "c"]);
    /// assert_eq!(
    ///     mset.threshold(1),
    ///     HashSet::from_iter(vec![&"a", &"b", &"c"])
    /// );
    /// assert_eq!(mset.threshold(2), HashSet::from_iter(vec![&"a"]));
    /// ```
    pub fn threshold(&self, threshold: u64) -> HashSet<&T> {
        self.threshold_iter(threshold).collect()
    }

    /// Returns an `Iterator` with the elements in the threshold.
    pub fn threshold_iter(&self, threshold: u64) -> impl Iterator<Item = &T> {
        self.occurrences
            .iter()
            .filter(move |(_, &count)| count >= threshold)
            .map(|(elem, _)| elem)
    }
}
