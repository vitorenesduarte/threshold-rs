#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use threshold::MultiSet;

#[quickcheck]
fn multiset_multiplicity(x: u64, ls: Vec<Vec<u64>>) -> bool {
    let mut mset = MultiSet::new();

    // add all lists of elements to the multiset
    for l in ls.clone() {
        mset.add(l);
    }

    // count the occurrences of `x`
    let count = ls.iter().flatten().filter(|&&y| y == x).count();

    count as u64 == mset.count(&x)
}
