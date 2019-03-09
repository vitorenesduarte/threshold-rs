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
fn add_and_count(x: u64, ls: Vec<Vec<u64>>) -> bool {
    let mset = mset(ls.clone());

    // prop: correct counting of `x`
    count(&x, &ls) == mset.count(&x)
}

#[quickcheck]
fn threshold(threshold: u64, ls: Vec<Vec<u64>>) -> bool {
    let mset = mset(ls.clone());

    // prop: all the elements have a count higher than the threshold
    mset.threshold(threshold)
        .iter()
        .all(|x| count(&x, &ls) >= threshold)
}

/// Create a `MultiSet<T>` from a vector of vectors.
fn mset<T: std::cmp::Ord>(ls: Vec<Vec<T>>) -> MultiSet<T> {
    let mut mset = MultiSet::new();

    // add all lists of elements to the multiset
    for l in ls {
        mset.add(l);
    }

    mset
}

/// Count the number of occurrences of `x` in the vector of vectors.
fn count(x: &u64, ls: &Vec<Vec<u64>>) -> u64 {
    ls.iter().flatten().filter(|&y| y == x).count() as u64
}
