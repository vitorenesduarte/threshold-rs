//! This module contains an implementation of a multi-set supporting a threshold
//! operation.
//!
//! The concept of threshold is explained in detail in [this blog post](https://vitorenes.org/post/2018/11/threshold-union/).
//!
//! # Examples
//! ```
//! use std::iter::FromIterator;
//! use threshold::*;
//!
//! let mut mset = MultiSet::new();
//!
//! mset.add(vec![(17, 1), (23, 1)]);
//! assert_eq!(mset.threshold(1), vec![&17, &23]);
//!
//! mset.add(vec![(17, 1), (42, 3)]);
//! assert_eq!(mset.threshold(1), vec![&17, &23, &42]);
//! assert_eq!(mset.threshold(2), vec![&17, &42]);
//! assert_eq!(mset.threshold(3), vec![&42]);
//! ```

use crate::Count;
use std::collections::btree_map::{self, BTreeMap};
use std::iter::FromIterator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiSet<E: Ord, C: Count> {
    /// Associate a count to each element
    occurrences: BTreeMap<E, C>,
}

impl<E: Ord, C: Count> MultiSet<E, C> {
    /// Returns a new `MultiSet` instance.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        MultiSet {
            occurrences: BTreeMap::new(),
        }
    }

    /// Creates a new `MultiSet` from an iterator of tuples (elem, elem count).
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mset = MultiSet::from(vec![(17, 1), (23, 2)]);
    /// assert_eq!(mset.count(&17), 1);
    /// assert_eq!(mset.count(&23), 2);
    /// ```
    pub fn from<I: IntoIterator<Item = (E, C)>>(iter: I) -> Self {
        MultiSet {
            occurrences: BTreeMap::from_iter(iter),
        }
    }

    /// Adds several elements (each with an associated count) to the `MultiSet`.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut mset = MultiSet::new();
    /// assert_eq!(mset.count(&17), 0);
    ///
    /// mset.add(vec![(17, 1), (23, 2)]);
    /// assert_eq!(mset.count(&17), 1);
    /// assert_eq!(mset.count(&23), 2);
    /// ```
    pub fn add<I: IntoIterator<Item = (E, C)>>(&mut self, iter: I) {
        for (elem, by) in iter {
            self.add_elem(elem, by);
        }
    }

    /// Adds a single element (with an associated count) to the `MultiSet`.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut mset = MultiSet::new();
    /// assert_eq!(mset.count(&17), 0);
    ///
    /// mset.add_elem(17, 2);
    /// assert_eq!(mset.count(&17), 2);
    /// ```
    pub fn add_elem(&mut self, elem: E, by: C) {
        // increase element count
        let count = self.occurrences.entry(elem).or_insert_with(Count::zero);
        count.add(by);
    }

    /// Returns the `Count` of an element.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut mset = MultiSet::new();
    /// assert_eq!(mset.count(&17), 0);
    ///
    /// mset.add(vec![(17, 1), (23, 1)]);
    /// assert_eq!(mset.count(&17), 1);
    /// assert_eq!(mset.count(&23), 1);
    /// assert_eq!(mset.count(&42), 0);
    ///
    /// mset.add(vec![(17, 1), (42, 1)]);
    /// assert_eq!(mset.count(&17), 2);
    /// assert_eq!(mset.count(&23), 1);
    /// assert_eq!(mset.count(&42), 1);
    /// assert_eq!(mset.count(&108), 0);
    /// ```
    pub fn count(&self, elem: &E) -> C {
        self.occurrences
            .get(elem)
            .map_or(Count::zero(), |&count| count)
    }

    /// Returns a sorted (ASC) double ended iterator.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = (&E, &C)> {
        self.occurrences.iter()
    }
}

impl<E: Ord> MultiSet<E, u64> {
    /// Returns the elements in the `MultiSet` such that its multiplicity is
    /// bigger or equal than a given [threshold](https://vitorenes.org/post/2018/11/threshold-union/).
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut mset = MultiSet::new();
    /// let empty: Vec<&u64> = Vec::new();
    /// assert_eq!(mset.threshold(1), empty);
    ///
    /// mset.add(vec![(17, 1), (23, 1)]);
    /// assert_eq!(mset.threshold(1), vec![&17, &23]);
    /// assert_eq!(mset.threshold(2), empty);
    ///
    /// mset.add(vec![(17, 1), (42, 3)]);
    /// assert_eq!(mset.threshold(1), vec![&17, &23, &42]);
    /// assert_eq!(mset.threshold(2), vec![&17, &42]);
    /// assert_eq!(mset.threshold(3), vec![&42]);
    /// assert_eq!(mset.threshold(4), empty);
    /// ```
    pub fn threshold(&self, threshold: u64) -> Vec<&E> {
        self.occurrences
            .iter()
            .filter(|(_, &count)| count >= threshold)
            .map(|(elem, _)| elem)
            .collect()
    }
}

pub struct IntoIter<E: Ord, C: Count>(btree_map::IntoIter<E, C>);

impl<E: Ord, C: Count> Iterator for IntoIter<E, C> {
    type Item = (E, C);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<E: Ord, C: Count> IntoIterator for MultiSet<E, C> {
    type Item = (E, C);
    type IntoIter = IntoIter<E, C>;

    /// Returns a `MultiSet` into iterator.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let elems_count = vec![("A", 2), ("B", 1)];
    /// let mset = MultiSet::from(elems_count);
    ///
    /// let mut iter = mset.into_iter();
    /// assert_eq!(Some(("A", 2)), iter.next());
    /// assert_eq!(Some(("B", 1)), iter.next());
    /// assert_eq!(None, iter.next());
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.occurrences.into_iter())
    }
}
