use crate::tests::arbitrary::Musk;
use crate::*;
use quickcheck_macros::quickcheck;

#[quickcheck]
fn next_dot(actor: Musk, vclock: VClock<Musk>) -> bool {
    let mut vclock = vclock.clone();
    let dot = vclock.next_dot(&actor);

    // prop: a newly created dot is now part of the clock
    vclock.is_element(&dot)
}

#[quickcheck]
fn add_dot(dot: Dot<Musk>, vclock: VClock<Musk>) -> bool {
    let mut vclock = vclock.clone();
    vclock.add_dot(&dot);

    // prop: a newly added dot is now part of the clock
    vclock.is_element(&dot)
}

#[quickcheck]
fn join(vclock_a: VClock<Musk>, vclock_b: VClock<Musk>) -> bool {
    let mut vclock_a = vclock_a.clone();
    vclock_a.join(&vclock_b);

    // prop: after merging b into a, all events in b are events in a
    vclock_b.into_iter().all(|(actor, eset)| {
        eset.into_iter().all(|seq| {
            let dot = Dot::new(&actor, seq);
            vclock_a.is_element(&dot)
        })
    })
}
