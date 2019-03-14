#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use threshold::*;

#[quickcheck]
fn next_dot(actor: String, vclock: VClock<String>) -> bool {
    let mut vclock = vclock.clone();
    let dot = vclock.next_dot(&actor);
    vclock.is_element(&dot)
}

#[quickcheck]
fn add_dot(dot: Dot<String>, vclock: VClock<String>) -> bool {
    let mut vclock = vclock.clone();
    vclock.add_dot(&dot);
    vclock.is_element(&dot)
}

#[quickcheck]
fn join(vclock_a: VClock<String>, vclock_b: VClock<String>) -> bool {
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
