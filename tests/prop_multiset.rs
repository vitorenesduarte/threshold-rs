#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use quickcheck::TestResult;
use threshold::MultiSet;

#[quickcheck]
fn singleton(x: u64, y: u64) -> TestResult {
    // discard invalid inputs
    if x == y {
        return TestResult::discard();
    }

    let mset = MultiSet::singleton(x);

    // prop: only the element in the singleton has count 1
    let prop = mset.count(&x) == 1 && mset.count(&y) == 0;
    TestResult::from_bool(prop)
}

#[quickcheck]
fn add_and_count(l: Vec<u64>, mset: MultiSet<u64>) -> bool {
    let mut new_mset = mset.clone();
    new_mset.add(l.clone());

    // prop: count of the element increased after add by the number of
    // occurrences of that element in `l`
    l.iter()
        .all(|x| new_mset.count(&x) == mset.count(&x) + count(&x, &l))
}

#[quickcheck]
fn threshold(threshold: u64, mset: MultiSet<u64>) -> bool {
    // prop: all the elements have a count higher than the threshold
    mset.threshold(threshold)
        .iter()
        .all(|x| mset.count(&x) >= threshold)
}

/// Count the number of occurrences of `x` in the vector of vectors.
fn count(x: &u64, ls: &Vec<u64>) -> u64 {
    ls.iter().filter(|&y| y == x).count() as u64
}