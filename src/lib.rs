// This module contains the definition of the `Actor` trait.
pub mod traits;

// This module contains the implementation of a Vector Clock.
pub mod vclock;

// This module contains the implementation of a Multi Set.
pub mod multiset;

// This module contains the implementation of Threshold Vector Clock.
pub mod tclock;

// This module contains implementations of quickcheck::Arbitrary trait.
mod arbitrary;

// Top-level re-exports.
pub use crate::multiset::MultiSet;
pub use crate::tclock::TClock;
pub use crate::traits::Actor;
pub use crate::vclock::{Dot, VClock};
