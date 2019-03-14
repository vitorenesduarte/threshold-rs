#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use quickcheck::TestResult;
use threshold::*;

#[quickcheck]
fn threshold_union(
    threshold: usize,
    dot: Dot<u64>,
    clocks: Vec<VClock<u64>>,
) -> TestResult {
    // pre: `threshold` is <= than the number of `clocks`
    if threshold > clocks.len() {
        return TestResult::discard();
    }

    // add all clocks to the threshold clock
    let mut tclock = TClock::new();
    for clock in clocks.clone() {
        tclock.add(clock);
    }

    // compute the threshold union
    let vclock = tclock.threshold_union(threshold as u64);

    // compute the number of occurrences of `dot` in `clocks`
    let occurrences =
        clocks.iter().filter(|clock| clock.is_element(&dot)).count();

    // prop: if the `dot` is in the resulting `vclock`, then its number of
    // occurrences is >= `threshold`
    let prop = if vclock.is_element(&dot) {
        occurrences >= threshold
    } else {
        occurrences < threshold
    };
    TestResult::from_bool(prop)
}
