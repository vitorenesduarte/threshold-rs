use crate::*;
use quickcheck::{Arbitrary, Gen};

const MAX_EVENTS: u64 = 20;

/// This enum should allow tests to be more effective since they only work on a
/// small number of actors.
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

    fn shrink(&self) -> Box<dyn Iterator<Item = Musk>> {
        Box::new(std::iter::empty::<Musk>())
    }
}

impl<E: Ord + Arbitrary, C: Count + Arbitrary> Arbitrary for MultiSet<E, C> {
    fn arbitrary<G: Gen>(g: &mut G) -> MultiSet<E, C> {
        let vec: Vec<(E, C)> = Arbitrary::arbitrary(g);
        MultiSet::from(vec)
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = MultiSet<E, C>>> {
        let vec: Vec<(E, C)> = self.clone().into_iter().collect();
        Box::new(vec.shrink().map(|v| MultiSet::from(v)))
    }
}

impl Arbitrary for MaxSet {
    fn arbitrary<G: Gen>(g: &mut G) -> MaxSet {
        let events: Vec<u64> = Arbitrary::arbitrary(g);
        MaxSet::from_events(events)
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = MaxSet>> {
        let vec: Vec<u64> = self.clone().into_iter().collect();
        Box::new(vec.shrink().map(|v| MaxSet::from_events(v)))
    }
}

impl Arbitrary for AboveExSet {
    fn arbitrary<G: Gen>(g: &mut G) -> AboveExSet {
        let events: Vec<u64> = Arbitrary::arbitrary(g);
        // reduce the number of possible events
        let events: Vec<u64> =
            events.into_iter().filter(|&x| x <= MAX_EVENTS).collect();
        AboveExSet::from_events(events)
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = AboveExSet>> {
        let vec: Vec<u64> = self.clone().into_iter().collect();
        Box::new(vec.shrink().map(|v| AboveExSet::from_events(v)))
    }
}

impl Arbitrary for BelowExSet {
    fn arbitrary<G: Gen>(g: &mut G) -> BelowExSet {
        let events: Vec<u64> = Arbitrary::arbitrary(g);
        // reduce the number of possible events
        let events: Vec<u64> =
            events.into_iter().filter(|&x| x <= MAX_EVENTS).collect();
        BelowExSet::from_events(events)
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = BelowExSet>> {
        let vec: Vec<u64> = self.clone().into_iter().collect();
        Box::new(vec.shrink().map(|v| BelowExSet::from_events(v)))
    }
}

impl<A: Actor + Arbitrary> Arbitrary for Dot<A> {
    fn arbitrary<G: Gen>(g: &mut G) -> Dot<A> {
        let actor: A = Arbitrary::arbitrary(g);
        let seq: u64 = Arbitrary::arbitrary(g);
        // ensure `seq` is never 0
        let seq = seq + 1;
        Dot::new(&actor, seq)
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Dot<A>>> {
        Box::new(std::iter::empty::<Dot<_>>())
    }
}

impl<A: Actor + Arbitrary, E: EventSet + Arbitrary> Arbitrary for Clock<A, E> {
    fn arbitrary<G: Gen>(g: &mut G) -> Clock<A, E> {
        let vec: Vec<(A, E)> = Arbitrary::arbitrary(g);
        Clock::from(vec)
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Clock<A, E>>> {
        let vec: Vec<(A, E)> = self.clone().into_iter().collect();
        Box::new(vec.shrink().map(|v| Clock::from(v)))
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use crate::tests::arbitrary::Musk;
    use quickcheck::{Arbitrary, StdThreadGen};

    const ITERATIONS: usize = 100;

    #[test]
    fn no_shrink() {
        no_shrink_assert::<Musk>();
        no_shrink_assert::<Dot<u64>>();
    }

    #[test]
    fn some_shrink() {
        some_shrink_assert::<MultiSet<u64, u64>>();
        some_shrink_assert::<MaxSet>();
        some_shrink_assert::<AboveExSet>();
        some_shrink_assert::<BelowExSet>();
        some_shrink_assert::<VClock<u64>>();
    }

    fn arbitrary<T: Arbitrary>() -> T {
        let mut g = StdThreadGen::new(100);
        Arbitrary::arbitrary(&mut g)
    }

    fn no_shrink_assert<T: Arbitrary>() {
        for _ in 0..ITERATIONS {
            let a = arbitrary::<T>();
            assert_eq!(a.shrink().count(), 0);
        }
    }

    fn some_shrink_assert<T: Arbitrary + IntoIterator>() {
        for _ in 0..ITERATIONS {
            let a = arbitrary::<T>();
            match a.clone().into_iter().count() {
                0 => (),
                _ => assert!(a.shrink().count() > 0),
            }
        }
    }
}
