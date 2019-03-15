#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use threshold::*;

#[quickcheck]
fn threshold_union(
    dot: Dot<Musk>,
    clock_a: VClock<Musk>,
    clock_b: VClock<Musk>,
    clock_c: VClock<Musk>,
) -> bool {
    // create a vec with all clocks
    let clocks = vec![clock_a, clock_b, clock_c];

    // add all clocks to the threshold clock
    let mut tclock = TClock::new();
    for clock in clocks.clone() {
        tclock.add(clock);
    }

    // create a vec with possible threshold values
    let thresholds = vec![1, 2, 3, 4];

    thresholds.into_iter().all(|threshold| {
        // compute the threshold union
        let vclock = tclock.threshold_union(threshold as u64);

        // compute the number of occurrences of `dot` in `clocks`
        let occurrences =
            clocks.iter().filter(|clock| clock.is_element(&dot)).count();

        // prop: if the `dot` is in the resulting `vclock`, then its number of
        // occurrences is >= `threshold`
        if vclock.is_element(&dot) {
            occurrences >= threshold
        } else {
            occurrences < threshold
        }
    })
}
