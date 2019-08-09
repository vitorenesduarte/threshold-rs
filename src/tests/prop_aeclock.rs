use crate::tests::arbitrary::Musk;
use crate::*;
use quickcheck_macros::quickcheck;

#[quickcheck]
fn next_dot(actor: Musk, aeclock: AEClock<Musk>) -> bool {
    let mut aeclock = aeclock.clone();
    let dot = aeclock.next_dot(&actor);

    // prop: a newly created dot is now part of the clock
    aeclock.is_element(&dot)
}

#[quickcheck]
fn add_dot(dot: Dot<Musk>, aeclock: AEClock<Musk>) -> bool {
    let mut aeclock = aeclock.clone();
    aeclock.add_dot(&dot);

    // prop: a newly added dot is now part of the clock
    aeclock.is_element(&dot)
}

#[quickcheck]
fn join(aeclock_a: AEClock<Musk>, aeclock_b: AEClock<Musk>) -> bool {
    let mut aeclock_a = aeclock_a.clone();
    aeclock_a.join(&aeclock_b);

    // prop: after merging b into a, all events in b are events in a
    aeclock_b.into_iter().all(|(actor, eset)| {
        eset.into_iter().all(|seq| {
            let dot = Dot::new(&actor, seq);
            aeclock_a.is_element(&dot)
        })
    })
}
