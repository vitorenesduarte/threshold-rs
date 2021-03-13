use crate::tests::arbitrary::Musk;
use crate::*;
use quickcheck_macros::quickcheck;
use std::collections::BTreeSet;

#[quickcheck]
fn add_dot(actor: Musk, event: u64, aeclock: AEClock<Musk>) -> bool {
    let mut aeclock = aeclock.clone();
    aeclock.add(&actor, event);

    // prop: a newly added dot is now part of the clock
    aeclock.contains(&actor, event)
}

#[quickcheck]
fn join(mut aeclock_a: AEClock<Musk>, aeclock_b: AEClock<Musk>) -> bool {
    aeclock_a.join(&aeclock_b);

    // prop: after merging b into a, all events in b are events in a
    aeclock_b.into_iter().all(|(actor, eset)| {
        eset.event_iter().all(|seq| aeclock_a.contains(&actor, seq))
    })
}

#[quickcheck]
fn meet(aeclock_a: AEClock<Musk>, aeclock_b: AEClock<Musk>) -> bool {
    let mut result = aeclock_a.clone();
    result.meet(&aeclock_b);

    aeclock_a.into_iter().all(|(actor, eset_a)| {
        let a = eset_a.event_iter().collect::<BTreeSet<_>>();
        let b = aeclock_b
            .get(&actor)
            .map(|eset_b| eset_b.clone().event_iter().collect::<BTreeSet<_>>())
            .unwrap_or_default();
        let expected = a.intersection(&b).cloned().collect::<BTreeSet<_>>();
        let result = result
            .get(&actor)
            .cloned()
            .unwrap_or_default()
            .event_iter()
            .collect();
        expected == result
    })
}

#[quickcheck]
fn subtracted(aeclock_a: AEClock<Musk>, aeclock_b: AEClock<Musk>) -> bool {
    let result = aeclock_a.subtracted(&aeclock_b);

    aeclock_a.into_iter().all(|(actor, eset_a)| {
        let a = eset_a.event_iter().collect::<BTreeSet<_>>();
        let b = aeclock_b
            .get(&actor)
            .map(|eset_b| eset_b.clone().event_iter().collect::<BTreeSet<_>>())
            .unwrap_or_default();
        let expected = a.difference(&b).cloned().collect::<BTreeSet<_>>();
        let result = result
            .get(&actor)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .collect();
            println!("a       : {:?}", a);
            println!("b       : {:?}", b);
            println!("expected: {:?}", expected);
            println!("result  : {:?}", result);
        expected == result
    })
}
