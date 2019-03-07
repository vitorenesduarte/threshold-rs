use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub struct TSet<T> {
    set_count: u64,
    occurrences: HashMap<T, u64>,
}

impl<T: Hash + Eq> TSet<T> {
    pub fn new() -> Self {
        TSet {
            set_count: 0,
            occurrences: HashMap::new(),
        }
    }

    pub fn set_count(&self) -> u64 {
        self.set_count
    }

    pub fn count(&self, elem: &T) -> u64 {
        self.occurrences.get(elem).map_or(0, |&count| count)
    }

    pub fn add<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.set_count += 1;

        for elem in iter {
            self.add_elem(elem);
        }
    }

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

    pub fn threshold_union(&self, threshold: u64) -> HashSet<&T> {
        self.occurrences
            .iter()
            .filter(|(_, &count)| count >= threshold)
            .map(|(elem, _)| elem)
            .collect()
    }

    pub fn union(&self) -> HashSet<&T> {
        self.threshold_union(1)
    }

    pub fn intersection(&self) -> HashSet<&T> {
        self.threshold_union(self.set_count)
    }
}

#[cfg(test)]
mod test {
    use super::TSet;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn set_count() {
        let mut tset = TSet::new();
        assert_eq!(tset.set_count(), 0);

        tset.add(vec!["a", "b"]);
        assert_eq!(tset.set_count(), 1);
    }

    #[test]
    fn count() {
        let mut tset = TSet::new();
        assert_eq!(tset.count(&"a"), 0);

        tset.add(vec!["a", "b"]);
        assert_eq!(tset.count(&"a"), 1);
        assert_eq!(tset.count(&"b"), 1);
        assert_eq!(tset.count(&"c"), 0);

        tset.add(vec!["a", "c"]);
        assert_eq!(tset.count(&"a"), 2);
        assert_eq!(tset.count(&"b"), 1);
        assert_eq!(tset.count(&"c"), 1);
        assert_eq!(tset.count(&"d"), 0);
    }

    #[test]
    fn threshold_union() {
        let mut tset = TSet::new();
        assert_eq!(tset.threshold_union(1), HashSet::new());

        tset.add(vec!["a", "b"]);
        assert_eq!(
            tset.threshold_union(1),
            HashSet::from_iter(vec![&"a", &"b"])
        );
        assert_eq!(tset.threshold_union(2), HashSet::new());

        tset.add(vec!["a", "c"]);
        assert_eq!(
            tset.threshold_union(1),
            HashSet::from_iter(vec![&"a", &"b", &"c"])
        );
        assert_eq!(tset.threshold_union(2), HashSet::from_iter(vec![&"a"]));
    }

    #[test]
    fn union_intersection() {
        let mut tset = TSet::new();
        assert_eq!(tset.union(), HashSet::new());
        assert_eq!(tset.intersection(), HashSet::new());

        tset.add(vec!["a", "b"]);
        assert_eq!(tset.union(), HashSet::from_iter(vec![&"a", &"b"]));
        assert_eq!(tset.intersection(), HashSet::from_iter(vec![&"a", &"b"]));

        tset.add(vec!["a", "c"]);
        assert_eq!(tset.union(), HashSet::from_iter(vec![&"a", &"b", &"c"]));
        assert_eq!(tset.intersection(), HashSet::from_iter(vec![&"a"]));
    }
}
