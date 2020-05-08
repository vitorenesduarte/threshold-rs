use crate::*;
use quickcheck::TestResult;
use quickcheck_macros::quickcheck;
use std::collections::BTreeSet;
use std::iter::FromIterator;

#[quickcheck]
fn add_event_above_exset(event: u64, events: BTreeSet<u64>) -> TestResult {
    check_add_event::<AboveExSet>(event, events)
}

#[quickcheck]
fn add_event_above_range_set(event: u64, events: BTreeSet<u64>) -> TestResult {
    check_add_event::<AboveRangeSet>(event, events)
}

#[quickcheck]
fn add_event_below_exset(event: u64, events: BTreeSet<u64>) -> TestResult {
    check_add_event::<BelowExSet>(event, events)
}

#[quickcheck]
fn add_event_range_above_exset(
    start: u64,
    end: u64,
    events: BTreeSet<u64>,
) -> TestResult {
    check_add_event_range::<AboveExSet>(start, end, events)
}

#[quickcheck]
fn add_event_range_above_range_set(
    start: u64,
    end: u64,
    events: BTreeSet<u64>,
) -> TestResult {
    check_add_event_range::<AboveRangeSet>(start, end, events)
}

#[quickcheck]
fn add_event_range_below_exset(
    start: u64,
    end: u64,
    events: BTreeSet<u64>,
) -> TestResult {
    check_add_event_range::<BelowExSet>(start, end, events)
}

#[quickcheck]
fn is_event_max_set(events: Vec<u64>) -> bool {
    check_is_event::<MaxSet>(events)
}

#[quickcheck]
fn is_event_above_exset(events: Vec<u64>) -> bool {
    check_is_event::<AboveExSet>(events)
}

#[quickcheck]
fn is_event_above_range_set(events: Vec<u64>) -> bool {
    check_is_event::<AboveRangeSet>(events)
}

#[quickcheck]
fn is_event_below_exset(events: Vec<u64>) -> bool {
    check_is_event::<BelowExSet>(events)
}

#[quickcheck]
fn join_max_set(events_a: Vec<u64>, events_b: Vec<u64>) -> bool {
    check_join::<MaxSet>(events_a, events_b)
}

#[quickcheck]
fn join_above_exset(events_a: Vec<u64>, events_b: Vec<u64>) -> bool {
    check_join::<AboveExSet>(events_a, events_b)
}

#[quickcheck]
fn join_above_range_set(events_a: Vec<u64>, events_b: Vec<u64>) -> bool {
    check_join::<AboveRangeSet>(events_a, events_b)
}

#[quickcheck]
fn join_below_exset(events_a: Vec<u64>, events_b: Vec<u64>) -> bool {
    check_join::<BelowExSet>(events_a, events_b)
}

#[quickcheck]
fn frontier_maxset(events: BTreeSet<u64>) -> bool {
    let eset = MaxSet::from_events(events.clone());
    let frontier = events.into_iter().last().unwrap_or(0);
    eset.frontier() == frontier
}

#[quickcheck]
fn frontier_above_exset(events: BTreeSet<u64>) -> TestResult {
    check_frontier::<AboveExSet>(events)
}

#[quickcheck]
fn frontier_above_range_set(events: BTreeSet<u64>) -> TestResult {
    check_frontier::<AboveRangeSet>(events)
}

#[quickcheck]
fn frontier_below_exset(events: BTreeSet<u64>) -> TestResult {
    check_frontier::<BelowExSet>(events)
}

#[quickcheck]
fn subtract_maxset(events: BTreeSet<u64>, subtract: BTreeSet<u64>) -> bool {
    check_subtract_maxset(events, subtract)
}

#[quickcheck]
fn subtract_above_exset_from_above_exset(
    events: BTreeSet<u64>,
    subtract: BTreeSet<u64>,
) -> bool {
    check_subtract::<AboveExSet, AboveExSet>(events, subtract)
}

#[quickcheck]
fn subtract_above_range_set_from_above_range_set(
    events: BTreeSet<u64>,
    subtract: BTreeSet<u64>,
) -> bool {
    check_subtract::<AboveRangeSet, AboveRangeSet>(events, subtract)
}

#[quickcheck]
fn subtract_above_exset_from_below_exset(
    events: BTreeSet<u64>,
    subtract: BTreeSet<u64>,
) -> bool {
    check_subtract::<AboveExSet, BelowExSet>(events, subtract)
}

#[quickcheck]
fn subtract_above_range_set_from_below_exset(
    events: BTreeSet<u64>,
    subtract: BTreeSet<u64>,
) -> bool {
    check_subtract::<AboveRangeSet, BelowExSet>(events, subtract)
}

#[quickcheck]
fn subtract_below_exset_from_above_exset(
    events: BTreeSet<u64>,
    subtract: BTreeSet<u64>,
) -> bool {
    check_subtract::<BelowExSet, AboveExSet>(events, subtract)
}

#[quickcheck]
fn subtract_below_exset_from_above_range_set(
    events: BTreeSet<u64>,
    subtract: BTreeSet<u64>,
) -> bool {
    check_subtract::<BelowExSet, AboveRangeSet>(events, subtract)
}

#[quickcheck]
fn subtract_below_exset_from_below_exset(
    events: BTreeSet<u64>,
    subtract: BTreeSet<u64>,
) -> bool {
    check_subtract::<BelowExSet, BelowExSet>(events, subtract)
}

// TODO this test currently will fail with `MaxSet` due to its special semantics
// (events do not need to be added to be part of the set)
fn check_add_event<E: EventSet>(
    event: u64,
    mut events: BTreeSet<u64>,
) -> TestResult {
    // event 0 is not allowed
    if event == 0 {
        return TestResult::discard();
    }

    // prune all events from `events` that are higher than `event`
    events = events.into_iter().filter(|&e| e > event).collect();

    // create event set from events
    let mut eset = E::from_events(events.clone());

    // check if `event` is part of the` events` added
    let res_0 = if events.contains(&event) {
        // if yes, then adding it again returns false
        !eset.add_event(event)
    } else {
        // else, returns true (it's a new event)
        eset.add_event(event)
    };

    // also add `event` to `events`
    events.insert(event);

    // check that only the initial events and the added event are events now
    let highest_event = events.iter().last().unwrap();
    let res_1 = (1..highest_event + 10).all(|event| {
        // if `event` part of `events` then it's part of `eset`
        // otherwise it's not part of `eset`
        if events.contains(&event) {
            eset.is_event(event)
        } else {
            !eset.is_event(event)
        }
    });

    TestResult::from_bool(res_0 && res_1)
}

fn check_add_event_range<E: EventSet>(
    start: u64,
    end: u64,
    mut events: BTreeSet<u64>,
) -> TestResult {
    // event 0 and invalid ranges are not allowed
    if start == 0 || end == 0 || start > end {
        return TestResult::discard();
    }

    // prune all events from `events` that are part of the range to be added
    events = events
        .into_iter()
        .filter(|&e| e < start || e > end)
        .collect();

    // create event set from events
    let mut eset = E::from_events(events.clone());

    // add event range to eset
    eset.add_event_range(start, end);

    // also add event range to `events`
    events.extend(BTreeSet::from_iter(start..=end));

    // check that only the initial events and the added event are events now
    let highest_event = events.iter().last().unwrap();
    let res = (1..highest_event + 10).all(|event| {
        // if `event` part of `events` then it's part of `eset`
        // otherwise it's not part of `eset`
        if events.contains(&event) {
            eset.is_event(event)
        } else {
            !eset.is_event(event)
        }
    });

    TestResult::from_bool(res)
}

fn check_is_event<E: EventSet>(events: Vec<u64>) -> bool {
    let eset = E::from_events(events.clone());
    events.into_iter().all(|event| eset.is_event(event))
}

fn check_join<E: EventSet>(events_a: Vec<u64>, events_b: Vec<u64>) -> bool {
    let mut eset_a = E::from_events(events_a.clone());
    let eset_b = E::from_events(events_b.clone());
    eset_a.join(&eset_b);
    events_a
        .into_iter()
        .chain(events_b.into_iter())
        .all(|event| eset_a.is_event(event))
}

fn check_frontier<E: EventSet>(mut events: BTreeSet<u64>) -> TestResult {
    // 0's are not allowed as events
    events.remove(&0);

    let eset = E::from_events(events.clone());
    let mut frontier = 0;
    let _: Vec<_> = events
        .into_iter()
        .skip_while(|&event| {
            if event == frontier + 1 {
                frontier = event;
                true
            } else {
                false
            }
        })
        .collect();
    TestResult::from_bool(eset.frontier() == frontier)
}

fn check_subtract_maxset(
    events: BTreeSet<u64>,
    subtract: BTreeSet<u64>,
) -> bool {
    // find maximal values
    let max_event = events.iter().max().map_or(0, |max| *max);
    let max_subtract = subtract.iter().max().map_or(0, |max| *max);

    // create event set from events
    let eset = MaxSet::from_events(events);

    // create subtract
    let subtract = MaxSet::from_events(subtract);

    // compute subtracted
    let subtracted: Vec<_> = crate::subtract_iter(eset, subtract).collect();

    // create expected
    let expected: Vec<_> =
        ((max_subtract + 1)..=max_event).into_iter().collect();

    subtracted == expected
}

fn check_subtract<E: EventSet, S: EventSet>(
    events: BTreeSet<u64>,
    subtract: BTreeSet<u64>,
) -> bool {
    // create expected
    let expected: Vec<_> = events
        .iter()
        .filter(|event| !subtract.contains(event))
        .filter(|event| **event != 0) // 0 is not a valid event
        .cloned()
        .collect();

    // create event set from events
    let eset = E::from_events(events.clone());

    // create event from subtracrt
    let subtract = S::from_events(subtract.clone());

    // compute subtracted
    let subtracted: Vec<_> = crate::subtract_iter(eset, subtract).collect();

    subtracted == expected
}
