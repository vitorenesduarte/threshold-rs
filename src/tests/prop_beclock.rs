use crate::tests::arbitrary::Musk;
use crate::*;
use quickcheck_macros::quickcheck;

#[quickcheck]
fn add_dot(actor: Musk, event: u64, beclock: BEClock<Musk>) -> bool {
    let mut beclock = beclock.clone();
    beclock.add(&actor, event);

    // prop: a newly added dot is now part of the clock
    beclock.contains(&actor, event)
}

#[quickcheck]
fn join(mut beclock_a: BEClock<Musk>, beclock_b: BEClock<Musk>) -> bool {
    beclock_a.join(&beclock_b);

    // prop: after merging b into a, all events in b are events in a
    beclock_b.into_iter().all(|(actor, eset)| {
        eset.event_iter().all(|seq| beclock_a.contains(&actor, seq))
    })
}
