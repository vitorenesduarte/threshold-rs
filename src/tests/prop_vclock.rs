use crate::tests::arbitrary::Musk;
use crate::*;
use quickcheck_macros::quickcheck;
use std::collections::BTreeSet;

#[quickcheck]
fn next(actor: Musk, vclock: VClock<Musk>) -> bool {
    let mut vclock = vclock.clone();
    let next = vclock.next(&actor);

    // prop: a newly created event is now part of the clock
    vclock.contains(&actor, next)
}

#[quickcheck]
fn add_dot(actor: Musk, event: u64, vclock: VClock<Musk>) -> bool {
    let mut vclock = vclock.clone();
    vclock.add(&actor, event);

    // prop: a newly added dot is now part of the clock
    vclock.contains(&actor, event)
}

#[quickcheck]
fn join(mut vclock_a: VClock<Musk>, vclock_b: VClock<Musk>) -> bool {
    vclock_a.join(&vclock_b);

    // prop: after merging b into a, all events in b are events in a
    vclock_b.into_iter().all(|(actor, eset)| {
        eset.event_iter().all(|seq| vclock_a.contains(&actor, seq))
    })
}

#[quickcheck]
fn meet(vclock_a: VClock<Musk>, vclock_b: VClock<Musk>) -> bool {
    let mut result = vclock_a.clone();
    result.meet(&vclock_b);

    vclock_a.into_iter().all(|(actor, eset_a)| {
        let a = eset_a.frontier();
        let b = vclock_b
            .get(&actor)
            .map(|eset_b| eset_b.frontier())
            .unwrap_or(0);
        let min = std::cmp::min(a, b);
        // prop: it contains the min but no more than that
        if min > 0 {
            result.contains(&actor, min) && !result.contains(&actor, min + 1)
        } else {
            true
        }
    })
}

#[quickcheck]
fn subtracted(vclock_a: VClock<Musk>, vclock_b: VClock<Musk>) -> bool {
    let result = vclock_a.subtracted(&vclock_b);

    vclock_a.into_iter().all(|(actor, eset_a)| {
        let a = eset_a.event_iter().collect::<BTreeSet<_>>();
        let b = vclock_b
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
        expected == result
    })
}
