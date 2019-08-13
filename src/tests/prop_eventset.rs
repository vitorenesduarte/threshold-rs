use crate::*;
use quickcheck::TestResult;
use quickcheck_macros::quickcheck;
use std::collections::BTreeSet;

#[quickcheck]
fn add_event_above_exset(event: u64, events: BTreeSet<u64>) -> TestResult {
    check_add_event::<AboveExSet>(event, events)
}

#[quickcheck]
fn add_event_below_exset(event: u64, events: BTreeSet<u64>) -> TestResult {
    check_add_event::<BelowExSet>(event, events)
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
fn frontier_below_exset(events: BTreeSet<u64>) -> TestResult {
    check_frontier::<BelowExSet>(events)
}

// TODO this test currently will fail with `MaxSet` due to its special semantics
// (events do not need to be added to be part of the set)
fn check_add_event<E: EventSet>(
    event: u64,
    events: BTreeSet<u64>,
) -> TestResult {
    // event 0 is not allowed
    if event == 0 {
        return TestResult::discard();
    }

    // create event set from events
    let mut eset = E::from_events(events.clone());

    // check if event is part of the events added
    let res = if events.contains(&event) {
        // if yes, then adding it again returns false
        !eset.add_event(event)
    } else {
        // else, returns true (it's a new event)
        eset.add_event(event)
    };

    TestResult::from_bool(res)
}

fn check_is_event<E: EventSet>(events: Vec<u64>) -> bool {
    let eset = E::from_events(events.clone());
    events.iter().all(|event| eset.is_event(event))
}

fn check_join<E: EventSet>(events_a: Vec<u64>, events_b: Vec<u64>) -> bool {
    let mut eset_a = E::from_events(events_a.clone());
    let eset_b = E::from_events(events_b.clone());
    eset_a.join(&eset_b);
    events_a
        .iter()
        .chain(events_b.iter())
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
