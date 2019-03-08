use std::fmt::Debug;
use std::hash::Hash;

/// Actor trait to be used in `VClock`'s or `TClock`'s.
pub trait Actor: Clone + Hash + Eq + Debug {}
impl<A: Clone + Hash + Eq + Debug> Actor for A {}
