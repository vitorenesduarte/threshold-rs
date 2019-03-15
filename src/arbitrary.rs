use crate::*;
use quickcheck::{Arbitrary, Gen};

/// This enum should allow tests to be more effective since they only work on a
/// small number of actors.
/// TODO move this to a module in `tests/`
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum Musk {
    A,
    B,
    C,
}

impl Arbitrary for Musk {
    fn arbitrary<G: Gen>(g: &mut G) -> Musk {
        let which: u64 = Arbitrary::arbitrary(g);
        match which % 3 {
            0 => Musk::A,
            1 => Musk::B,
            _ => Musk::C,
        }
    }

    fn shrink(&self) -> Box<Iterator<Item = Musk>> {
        Box::new(std::iter::empty::<Musk>())
    }
}

impl<E: Ord + Arbitrary, C: Count + Arbitrary> Arbitrary for MultiSet<E, C> {
    fn arbitrary<G: Gen>(g: &mut G) -> MultiSet<E, C> {
        let vec: Vec<(E, C)> = Arbitrary::arbitrary(g);
        MultiSet::from(vec)
    }

    fn shrink(&self) -> Box<Iterator<Item = MultiSet<E, C>>> {
        let vec: Vec<(E, C)> = self.clone().into_iter().collect();
        Box::new(vec.shrink().map(|v| MultiSet::from(v)))
    }
}

impl Arbitrary for MaxSet {
    fn arbitrary<G: Gen>(g: &mut G) -> MaxSet {
        let events: Vec<u64> = Arbitrary::arbitrary(g);
        MaxSet::from_events(events)
    }

    fn shrink(&self) -> Box<Iterator<Item = MaxSet>> {
        let vec: Vec<u64> = self.clone().into_iter().collect();
        Box::new(vec.shrink().map(|v| MaxSet::from_events(v)))
    }
}

impl Arbitrary for BelowExSet {
    fn arbitrary<G: Gen>(g: &mut G) -> BelowExSet {
        let events: Vec<u64> = Arbitrary::arbitrary(g);
        BelowExSet::from_events(events)
    }

    fn shrink(&self) -> Box<Iterator<Item = BelowExSet>> {
        let vec: Vec<u64> = self.clone().into_iter().collect();
        Box::new(vec.shrink().map(|v| BelowExSet::from_events(v)))
    }
}

impl<A: Actor + Arbitrary> Arbitrary for Dot<A> {
    fn arbitrary<G: Gen>(g: &mut G) -> Dot<A> {
        let actor: A = Arbitrary::arbitrary(g);
        let seq: u64 = Arbitrary::arbitrary(g);
        let seq = seq + 1; // ensure it's never 0
        Dot::new(&actor, seq)
    }

    fn shrink(&self) -> Box<Iterator<Item = Dot<A>>> {
        Box::new(std::iter::empty::<Dot<_>>())
    }
}

impl<A: Actor + Arbitrary, E: EventSet + Arbitrary> Arbitrary for Clock<A, E> {
    fn arbitrary<G: Gen>(g: &mut G) -> Clock<A, E> {
        let vec: Vec<(A, E)> = Arbitrary::arbitrary(g);
        Clock::from(vec)
    }

    fn shrink(&self) -> Box<Iterator<Item = Clock<A, E>>> {
        let vec: Vec<(A, E)> = self.clone().into_iter().collect();
        Box::new(vec.shrink().map(|v| Clock::from(v)))
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
