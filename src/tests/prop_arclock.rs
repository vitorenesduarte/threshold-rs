use crate::tests::arbitrary::Musk;
use crate::*;
use quickcheck_macros::quickcheck;

#[quickcheck]
fn add_dot(actor: Musk, event: u64, arclock: ARClock<Musk>) -> bool {
    let mut aeclock = arclock.clone();
    aeclock.add(&actor, event);

    // prop: a newly added dot is now part of the clock
    aeclock.contains(&actor, event)
}

#[quickcheck]
fn join(arclock_a: ARClock<Musk>, aeclock_b: ARClock<Musk>) -> bool {
    let mut arclock_a = arclock_a.clone();
    arclock_a.join(&aeclock_b);

    // prop: after merging b into a, all events in b are events in a
    aeclock_b.into_iter().all(|(actor, eset)| {
        eset.event_iter().all(|seq| arclock_a.contains(&actor, seq))
    })
}
