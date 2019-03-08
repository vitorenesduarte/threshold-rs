[![Build Status](https://travis-ci.org/vitorenesduarte/threshold-rs.svg?branch=master)](https://travis-ci.org/vitorenesduarte/threshold-rs)
[![codecov](https://codecov.io/gh/vitorenesduarte/threshold-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/vitorenesduarte/threshold-rs)

### `threshold-rs`: threshold data structures in Rust!

#### Example
Assume multiset `X` is `{10: 1, 8: 2, 6: 3, 5: 1}`.
This means that event `10` was seen once, event `8` twice, and so on.

Assume that these events come from vector clocks, and thus seeing event 10 means seeing all events from 1 to 10.

If, for example, we want the event that was seen at least 4 times (i.e. our [threshold](https://vitorenes.org/post/2018/11/threshold-union/) is 4), we should get event `6`.
    
Assume `threshold(u64, X) -> Option<u64>` where the first argument is the threshold desired. Then:
- `threshold(1, X) = Some(10)`
- `threshold(2, X) = Some(8)`
- `threshold(3, X) = Some(8)`
- `threshold(4, X) = Some(6)`
- `threshold(7, X) = Some(5)`
- `threshold(8, X) = None`

#### Code Example
```rust
use threshold::{vclock, *};

let vclock_0 = vclock::from_seqs(vec![10, 5, 5]);
let vclock_1 = vclock::from_seqs(vec![8, 10, 6]);
let vclock_2 = vclock::from_seqs(vec![9, 8, 7]);

let mut tclock = TClock::new();
tclock.add(vclock_0);
tclock.add(vclock_1);
tclock.add(vclock_2);

let vclock_t1 = vclock::from_seqs(vec![10, 10, 7]);
let vclock_t2 = vclock::from_seqs(vec![9, 8, 6]);
let vclock_t3 = vclock::from_seqs(vec![8, 5, 5]);

assert_eq!(tclock.threshold_union(1), vclock_t1);
assert_eq!(tclock.threshold_union(2), vclock_t2);
assert_eq!(tclock.threshold_union(3), vclock_t3);
```
