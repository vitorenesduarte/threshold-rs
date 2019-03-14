use crate::*;
use quickcheck::{Arbitrary, Gen};

impl<E: Ord + Arbitrary, C: Count + Arbitrary> Arbitrary for MultiSet<E, C> {
    fn arbitrary<G: Gen>(g: &mut G) -> MultiSet<E, C> {
        let vec: Vec<(E, C)> = Arbitrary::arbitrary(g);
        MultiSet::from_vec(vec)
    }

    fn shrink(&self) -> Box<Iterator<Item = MultiSet<E, C>>> {
        let vec: Vec<(E, C)> = self.clone().into_iter().collect();
        Box::new(vec.shrink().map(|v| MultiSet::from_vec(v)))
    }
}

impl Arbitrary for MaxInt {
    fn arbitrary<G: Gen>(g: &mut G) -> MaxInt {
        let events: Vec<u64> = Arbitrary::arbitrary(g);
        MaxInt::from_events(events)
    }

    fn shrink(&self) -> Box<Iterator<Item = MaxInt>> {
        let vec: Vec<u64> = self.clone().into_iter().collect();
        Box::new(vec.shrink().map(|v| MaxInt::from_events(v)))
    }
}

impl<A: Actor + Arbitrary> Arbitrary for Dot<A> {
    fn arbitrary<G: Gen>(g: &mut G) -> Dot<A> {
        let actor: A = Arbitrary::arbitrary(g);
        let seq: u64 = Arbitrary::arbitrary(g);
        Dot::new(&actor, seq)
    }

    fn shrink(&self) -> Box<Iterator<Item = Dot<A>>> {
        Box::new(std::iter::empty::<Dot<_>>())
    }
}

impl<A: Actor + Arbitrary, E: EventSet + Arbitrary> Arbitrary for Clock<A, E> {
    fn arbitrary<G: Gen>(g: &mut G) -> Clock<A, E> {
        let vec: Vec<(A, E)> = Arbitrary::arbitrary(g);
        Clock::from_vec(vec)
    }

    fn shrink(&self) -> Box<Iterator<Item = Clock<A, E>>> {
        let vec: Vec<(A, E)> = self.clone().into_iter().collect();
        Box::new(vec.shrink().map(|v| Clock::from_vec(v)))
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use quickcheck::{Arbitrary, StdThreadGen};

    #[test]
    fn multiset_shrink() {
        let count = shrink_count::<MultiSet<u64, u64>>();
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
