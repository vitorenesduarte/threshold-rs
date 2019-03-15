#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use threshold::*;

#[quickcheck]
fn next_dot(actor: String, beclock: BEClock<String>) -> bool {
    let mut beclock = beclock.clone();
    let dot = beclock.next_dot(&actor);

    // prop: a newly created dot is now part of the clock
    beclock.is_element(&dot)
}

#[quickcheck]
fn add_dot(dot: Dot<String>, beclock: BEClock<String>) -> bool {
    let mut beclock = beclock.clone();
    beclock.add_dot(&dot);

    // prop: a newly added dot is now part of the clock
    beclock.is_element(&dot)
}

#[quickcheck]
fn join(beclock_a: BEClock<String>, beclock_b: BEClock<String>) -> bool {
    let mut beclock_a = beclock_a.clone();
    beclock_a.join(&beclock_b);

    // prop: after merging b into a, all events in b are events in a
    beclock_b.into_iter().all(|(actor, eset)| {
        eset.into_iter().all(|seq| {
            let dot = Dot::new(&actor, seq);
            beclock_a.is_element(&dot)
        })
    })
}
