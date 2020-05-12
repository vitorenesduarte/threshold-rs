use crate::tests::arbitrary::Musk;
use crate::*;
use quickcheck_macros::quickcheck;

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
