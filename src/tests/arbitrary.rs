use crate::*;
use quickcheck::{Arbitrary, Gen};

const MAX_EVENTS: u64 = 20;

/// This enum should allow tests to be more effective since they only work on a
/// small number of actors.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
        let vec: Vec<u64> = self.clone().event_iter().collect();
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
        let vec: Vec<u64> = self.clone().event_iter().collect();
        Box::new(vec.shrink().map(|v| AboveExSet::from_events(v)))
    }
}

impl Arbitrary for AboveRangeSet {
    fn arbitrary<G: Gen>(g: &mut G) -> AboveRangeSet {
        let events: Vec<u64> = Arbitrary::arbitrary(g);
        // reduce the number of possible events
        let events: Vec<u64> =
            events.into_iter().filter(|&x| x <= MAX_EVENTS).collect();
        AboveRangeSet::from_events(events)
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = AboveRangeSet>> {
        let vec: Vec<u64> = self.clone().event_iter().collect();
        Box::new(vec.shrink().map(|v| AboveRangeSet::from_events(v)))
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
        let vec: Vec<u64> = self.clone().event_iter().collect();
        Box::new(vec.shrink().map(|v| BelowExSet::from_events(v)))
    }
}

impl<A: Actor + Arbitrary, E: EventSet + Arbitrary> Arbitrary for Clock<A, E> {
    fn arbitrary<G: Gen>(g: &mut G) -> Clock<A, E> {
        let vec: Vec<(A, E)> = Arbitrary::arbitrary(g);
        Clock::from(vec)
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Clock<A, E>>> {
        Box::new(std::iter::empty())
        // TODO the following implementation leads to a stack overflow
        // create a vector with all events in the clock
        // let vec: Vec<(A, u64)> = self
        //     .clone()
        //     .into_iter()
        //     .flat_map(|(actor, eset)| {
        //         // TODO why is the move needed?
        //         eset.event_iter().map(move |event| (actor.clone(), event))
        //     })
        //     .collect();
        // Box::new(vec.shrink().map(|v| {
        //     let mut clock = Clock::new();
        //     for (actor, event) in v {
        //         clock.add(&actor, event);
        //     }
        //     clock
        // }))
    }
}

#[cfg(test)]
mod test {
    use crate::tests::arbitrary::Musk;
    use crate::*;
    use quickcheck::{Arbitrary, StdThreadGen};

    const ITERATIONS: usize = 100;

    #[test]
    fn no_shrink() {
        no_shrink_assert::<Musk>();
    }

    #[test]
    fn some_shrink() {
        some_shrink_assert::<MaxSet>();
        some_shrink_assert::<AboveExSet>();
        some_shrink_assert::<BelowExSet>();
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

    fn some_shrink_assert<T: Arbitrary + EventSet>() {
        for _ in 0..ITERATIONS {
            let a = arbitrary::<T>();
            match a.clone().event_iter().count() {
                0 => (),
                _ => assert!(a.shrink().count() > 0),
            }
        }
    }
}
