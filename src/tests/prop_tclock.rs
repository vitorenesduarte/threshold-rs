use crate::tests::arbitrary::Musk;
use crate::*;
use quickcheck::TestResult;
use quickcheck_macros::quickcheck;

#[quickcheck]
fn vclock_threshold_union(
    actor: Musk,
    event: u64,
    clock_a: VClock<Musk>,
    clock_b: VClock<Musk>,
    clock_c: VClock<Musk>,
) -> TestResult {
    // event 0 is not allowed
    if event == 0 {
        return TestResult::discard();
    }

    // create a vec with all clocks
    let clocks = vec![clock_a, clock_b, clock_c];

    // add all clocks to the threshold clock
    let mut tclock = TClock::new();
    for clock in clocks.clone() {
        tclock.add(clock);
    }

    // create a vec with possible threshold values
    let thresholds = vec![1, 2, 3, 4];

    let result = thresholds.into_iter().all(|threshold| {
        // compute the threshold union
        let (clock, equal_to_union) = tclock.threshold_union(threshold as u64);

        // prop: if threshold is 1, then threshold union must be the same as
        // union
        let result1 = if threshold == 1 { equal_to_union } else { true };

        // compute the number of occurrences of `dot` in `clocks`
        let occurrences = clocks
            .iter()
            .filter(|clock| clock.contains(&actor, event))
            .count();

        // prop: if the `dot` is in the resulting `clock`, then its number of
        // occurrences is >= `threshold`
        let result2 = if clock.contains(&actor, event) {
            occurrences >= threshold
        } else {
            occurrences < threshold
        };

        result1 && result2
    });

    TestResult::from_bool(result)
}

#[quickcheck]
fn vclock_union(clock_a: VClock<Musk>, clock_b: VClock<Musk>) -> TestResult {
    // add all clocks to the threshold clock
    let mut tclock = TClock::new();
    tclock.add(clock_a.clone());
    tclock.add(clock_b.clone());

    // compute union
    let (clock, all_equal) = tclock.union();

    let result = if clock_a == clock_b {
        // if the clocks are equal, then the resulting clock should be equal as well and the flag
        // `all_equal` be true
        clock == clock_a && all_equal
    } else {
        true
    };
    TestResult::from_bool(result)
}

#[quickcheck]
fn beclock_threshold_union(
    actor: Musk,
    event: u64,
    clock_a: BEClock<Musk>,
    clock_b: BEClock<Musk>,
    clock_c: BEClock<Musk>,
) -> TestResult {
    // event 0 is not allowed
    if event == 0 {
        return TestResult::discard();
    }
    // create a vec with all clocks
    let clocks = vec![clock_a, clock_b, clock_c];

    // add all clocks to the threshold clock
    let mut tclock = TClock::new();
    for clock in clocks.clone() {
        tclock.add(clock);
    }

    // create a vec with possible threshold values
    let thresholds = vec![1, 2, 3, 4];

    let result = thresholds.into_iter().all(|threshold| {
        // compute the threshold union
        let clock = tclock.threshold_union(threshold as u64);

        // compute the number of occurrences of `dot` in `clocks`
        let occurrences = clocks
            .iter()
            .filter(|clock| clock.contains(&actor, event))
            .count();

        // prop: if the `dot` is in the resulting `clock`, then its number of
        // occurrences is >= `threshold`
        if clock.contains(&actor, event) {
            occurrences >= threshold
        } else {
            occurrences < threshold
        }
    });

    TestResult::from_bool(result)
}
