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
//! mset.add(vec![17, 23]);
//! assert_eq!(mset.threshold(1), vec![&17, &23]);
//!
//! mset.add(vec![17, 42]);
//! assert_eq!(mset.threshold(1), vec![&17, &23, &42]);
//! assert_eq!(mset.threshold(2), vec![&17]);
//! ```

use std::collections::BTreeMap;

pub struct MultiSet<T: Ord> {
    /// Number of occurrences of each element added
    occurrences: BTreeMap<T, u64>,
}

impl<T: Ord> MultiSet<T> {
    /// Returns a new `MultiSet` instance.
    pub fn new() -> Self {
        MultiSet {
            occurrences: BTreeMap::new(),
        }
    }

    /// Returns a new `MultiSet` with a single element.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mset = MultiSet::singleton(17);
    /// assert_eq!(mset.count(&17), 1);
    /// ```
    pub fn singleton(elem: T) -> Self {
        let mut mset = Self::new();
        mset.add_elem(elem);
        mset
    }

    /// Adds several elements to the `MultiSet`.
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut mset = MultiSet::new();
    /// assert_eq!(mset.count(&17), 0);
    ///
    /// mset.add(vec![17, 23]);
    /// assert_eq!(mset.count(&17), 1);
    /// assert_eq!(mset.count(&23), 1);
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
    /// assert_eq!(mset.count(&17), 0);
    ///
    /// mset.add_elem(17);
    /// assert_eq!(mset.count(&17), 1);
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
    /// assert_eq!(mset.count(&17), 0);
    ///
    /// mset.add(vec![17, 23]);
    /// assert_eq!(mset.count(&17), 1);
    /// assert_eq!(mset.count(&23), 1);
    /// assert_eq!(mset.count(&42), 0);
    ///
    /// mset.add(vec![17, 42]);
    /// assert_eq!(mset.count(&17), 2);
    /// assert_eq!(mset.count(&23), 1);
    /// assert_eq!(mset.count(&42), 1);
    /// assert_eq!(mset.count(&108), 0);
    /// ```
    pub fn count(&self, elem: &T) -> u64 {
        self.occurrences.get(elem).map_or(0, |&count| count)
    }

    /// Returns the elements in the `MultiSet` such that its multiplicity is bigger or equal than a given [threshold](https://vitorenes.org/post/2018/11/threshold-union/).
    ///
    /// # Examples
    /// ```
    /// use threshold::*;
    ///
    /// let mut mset = MultiSet::new();
    /// let empty: Vec<&u64> = Vec::new();
    /// assert_eq!(mset.threshold(1), empty);
    ///
    /// mset.add(vec![17, 23]);
    /// assert_eq!(mset.threshold(1), vec![&17, &23]);
    /// assert_eq!(mset.threshold(2), empty);
    ///
    /// mset.add(vec![17, 42]);
    /// assert_eq!(mset.threshold(1), vec![&17, &23, &42]);
    /// assert_eq!(mset.threshold(2), vec![&17]);
    /// ```
    pub fn threshold(&self, threshold: u64) -> Vec<&T> {
        self.occurrences
            .iter()
            .filter(|(_, &count)| count >= threshold)
            .map(|(elem, _)| elem)
            .collect()
    }

    /// Returns a sorted (ASC) double ended iterator.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = (&T, &u64)> {
        self.occurrences.iter()
    }
}
