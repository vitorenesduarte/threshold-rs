use crate::*;
use quickcheck::{Arbitrary, Gen};

impl<T: Ord + Arbitrary> Arbitrary for MultiSet<T> {
    fn arbitrary<G: Gen>(g: &mut G) -> MultiSet<T> {
        let vec: Vec<T> = Arbitrary::arbitrary(g);
        let mut mset = MultiSet::new();
        mset.add(vec);
        mset
    }

    fn shrink(&self) -> Box<Iterator<Item = MultiSet<T>>> {
        let vec: Vec<(T, u64)> = self.clone().into_iter().collect();
        Box::new(vec.shrink().map(|v| MultiSet::from_vec(v)))
    }
}

impl<T: Actor + Arbitrary> Arbitrary for Dot<T> {
    fn arbitrary<G: Gen>(g: &mut G) -> Dot<T> {
        let actor: T = Arbitrary::arbitrary(g);
        let seq: u64 = Arbitrary::arbitrary(g);
        Dot::new(&actor, seq)
    }

    fn shrink(&self) -> Box<Iterator<Item = Dot<T>>> {
        Box::new(std::iter::empty::<Dot<_>>())
    }
}

impl<T: Actor + Arbitrary> Arbitrary for VClock<T> {
    fn arbitrary<G: Gen>(g: &mut G) -> VClock<T> {
        let vec: Vec<(T, u64)> = Arbitrary::arbitrary(g);
        VClock::from_vec(vec)
    }

    fn shrink(&self) -> Box<Iterator<Item = VClock<T>>> {
        let vec: Vec<(T, u64)> = self.clone().into_iter().collect();
        Box::new(vec.shrink().map(|v| VClock::from_vec(v)))
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use quickcheck::{Arbitrary, StdThreadGen};

    #[test]
    fn multiset_shrink() {
        let count = shrink_count::<MultiSet<u64>>();
        assert!(count > 0);
    }

    #[test]
    fn dot_shrink() {
        let count = shrink_count::<Dot<u64>>();
        assert!(count == 0);
    }

    #[test]
    fn vclock_shrink() {
        let count = shrink_count::<VClock<u64>>();
        assert!(count > 0);
    }

    fn shrink_count<T: Arbitrary>() -> usize {
        let mut g = StdThreadGen::new(100);
        let instance: T = Arbitrary::arbitrary(&mut g);
        instance.shrink().count()
    }
}
