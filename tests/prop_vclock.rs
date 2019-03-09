#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use threshold::*;

#[quickcheck]
fn next_dot(actor: u64, vclock: VClock<u64>) -> bool {
    let mut vclock = vclock.clone();
    let dot = vclock.next_dot(&actor);
    vclock.is_element(&dot)
}

#[quickcheck]
fn add_dot(dot: Dot<u64>, vclock: VClock<u64>) -> bool {
    let mut vclock = vclock.clone();
    vclock.add_dot(&dot);
    vclock.is_element(&dot)
}
