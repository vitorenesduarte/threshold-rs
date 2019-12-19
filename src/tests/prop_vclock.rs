use crate::tests::arbitrary::Musk;
use crate::*;
use quickcheck_macros::quickcheck;

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
fn join(vclock_a: VClock<Musk>, vclock_b: VClock<Musk>) -> bool {
    let mut vclock_a = vclock_a.clone();
    vclock_a.join(&vclock_b);

    // prop: after merging b into a, all events in b are events in a
    vclock_b.into_iter().all(|(actor, eset)| {
        eset.event_iter().all(|seq| vclock_a.contains(&actor, seq))
    })
}
