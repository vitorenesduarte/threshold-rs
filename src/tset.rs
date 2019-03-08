//! This module contains an implementation of a threshold set.
//!
//! The concept of threshold-union is explained in detail in [this blog post](https://vitorenes.org/post/2018/11/threshold-union/).
//!
//! # Examples
//! ```
//! use std::collections::HashSet;
//! use std::iter::FromIterator;
//! use threshold::*;
//!
//! let mut tset = TSet::new();
//! assert_eq!(tset.threshold_union(1), HashSet::new());
//!
//! tset.add(vec!["a", "b"]);
//! assert_eq!(
//!     tset.threshold_union(1),
//!     HashSet::from_iter(vec![&"a", &"b"])
//! );
//! assert_eq!(tset.threshold_union(2), HashSet::new());
//!
//! tset.add(vec!["a", "c"]);
//! assert_eq!(
//!     tset.threshold_union(1),
//!     HashSet::from_iter(vec![&"a", &"b", &"c"])
//! );
//! assert_eq!(tset.threshold_union(2), HashSet::from_iter(vec![&"a"]));
//! ```

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub struct TSet<T: Hash + Eq> {
    /// Number of sets added to the `TSet`
    set_count: u64,
    /// Number of occurrences of each element added
    occurrences: HashMap<T, u64>,
}

impl<T: Hash + Eq> TSet<T> {
    /// Returns a new `TSet` instance.
    pub fn new() -> Self {
        TSet {
            set_count: 0,
            occurrences: HashMap::new(),
        }
    }

    /// Adds a set of elements to the `TSet`.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut tset = TSet::new();
    /// assert_eq!(tset.set_count(), 0);
    ///
    /// tset.add(vec!["a", "b"]);
    /// assert_eq!(tset.set_count(), 1);
    /// ```
    pub fn add<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.set_count += 1;

        for elem in iter {
            self.add_elem(elem);
        }
    }

    /// Adds a single element to the `TSet`.
    fn add_elem(&mut self, elem: T) {
        match self.occurrences.get_mut(&elem) {
            Some(count) => {
                *count += 1;
            }
            None => {
                self.occurrences.insert(elem, 1);
            }
        }
    }

    /// Returns the number of sets added to the `TSet`.
    pub fn set_count(&self) -> u64 {
        self.set_count
    }

    /// Returns the number of occurrences of an element.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut tset = TSet::new();
    /// assert_eq!(tset.count(&"a"), 0);
    ///
    /// tset.add(vec!["a", "b"]);
    /// assert_eq!(tset.count(&"a"), 1);
    /// assert_eq!(tset.count(&"b"), 1);
    /// assert_eq!(tset.count(&"c"), 0);
    ///
    /// tset.add(vec!["a", "c"]);
    /// assert_eq!(tset.count(&"a"), 2);
    /// assert_eq!(tset.count(&"b"), 1);
    /// assert_eq!(tset.count(&"c"), 1);
    /// assert_eq!(tset.count(&"d"), 0);
    /// ```
    pub fn count(&self, elem: &T) -> u64 {
        self.occurrences.get(elem).map_or(0, |&count| count)
    }

    /// Computes the [threshold-union](https://vitorenes.org/post/2018/11/threshold-union/) of all sets added to the `TSet`.
    ///
    /// # Examples
    /// ```
    /// use std::collections::HashSet;
    /// use std::iter::FromIterator;
    /// use threshold::*;
    ///
    /// let mut tset = TSet::new();
    /// assert_eq!(tset.threshold_union(1), HashSet::new());
    ///
    /// tset.add(vec!["a", "b"]);
    /// assert_eq!(
    ///     tset.threshold_union(1),
    ///     HashSet::from_iter(vec![&"a", &"b"])
    /// );
    /// assert_eq!(tset.threshold_union(2), HashSet::new());
    ///
    /// tset.add(vec!["a", "c"]);
    /// assert_eq!(
    ///     tset.threshold_union(1),
    ///     HashSet::from_iter(vec![&"a", &"b", &"c"])
    /// );
    /// assert_eq!(tset.threshold_union(2), HashSet::from_iter(vec![&"a"]));
    /// ```
    pub fn threshold_union(&self, threshold: u64) -> HashSet<&T> {
        self.threshold_union_iter(threshold).collect()
    }

    /// Returns an `Iterator` with elements in the threshold-union.
    pub fn threshold_union_iter(
        &self,
        threshold: u64,
    ) -> impl Iterator<Item = &T> {
        self.occurrences
            .iter()
            .filter(move |(_, &count)| count >= threshold)
            .map(|(elem, _)| elem)
    }

    /// Computes the union of all sets added to the `TSet`.
    ///
    /// # Examples
    /// ```
    /// use std::collections::HashSet;
    /// use std::iter::FromIterator;
    /// use threshold::*;
    ///
    /// let mut tset = TSet::new();
    /// assert_eq!(tset.union(), HashSet::new());
    ///
    /// tset.add(vec!["a", "b"]);
    /// assert_eq!(tset.union(), HashSet::from_iter(vec![&"a", &"b"]));
    ///
    /// tset.add(vec!["a", "c"]);
    /// assert_eq!(tset.union(), HashSet::from_iter(vec![&"a", &"b", &"c"]));
    /// ```
    pub fn union(&self) -> HashSet<&T> {
        self.threshold_union(1)
    }

    /// Computes the intersection of all sets added to the `TSet`.
    ///
    /// # Examples
    /// ```
    /// use std::collections::HashSet;
    /// use std::iter::FromIterator;
    /// use threshold::*;
    ///
    /// let mut tset = TSet::new();
    /// assert_eq!(tset.intersection(), HashSet::new());
    ///
    /// tset.add(vec!["a", "b"]);
    /// assert_eq!(tset.intersection(), HashSet::from_iter(vec![&"a", &"b"]));
    ///
    /// tset.add(vec!["a", "c"]);
    /// assert_eq!(tset.intersection(), HashSet::from_iter(vec![&"a"]));
    /// ```
    pub fn intersection(&self) -> HashSet<&T> {
        self.threshold_union(self.set_count)
    }
}
