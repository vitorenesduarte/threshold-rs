use crate::tests::arbitrary::Musk;
use crate::*;
use quickcheck_macros::quickcheck;

#[quickcheck]
fn next_dot(actor: Musk, aeclock: AEClock<Musk>) -> bool {
    let mut aeclock = aeclock.clone();
    let event = aeclock.next(&actor);

    // prop: a newly created dot is now part of the clock
    aeclock.contains(&actor, event)
}

#[quickcheck]
fn add_dot(actor: Musk, event: u64, aeclock: AEClock<Musk>) -> bool {
    let mut aeclock = aeclock.clone();
    aeclock.add(&actor, event);

    // prop: a newly added dot is now part of the clock
    aeclock.contains(&actor, event)
}

#[quickcheck]
fn join(aeclock_a: AEClock<Musk>, aeclock_b: AEClock<Musk>) -> bool {
    let mut aeclock_a = aeclock_a.clone();
    aeclock_a.join(&aeclock_b);

    // prop: after merging b into a, all events in b are events in a
    aeclock_b.into_iter().all(|(actor, eset)| {
        eset.event_iter().all(|seq| aeclock_a.contains(&actor, seq))
    })
}
