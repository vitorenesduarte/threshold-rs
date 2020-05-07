// This module contains the definition of `Count`, `Actor` and `EventSet`
// traits.
mod traits;

// This module contains implementations of the `EventSet` trait.
mod set;

// This module contains the implementation of a Clock.
pub mod clock;

// This module contains the implementation of a Multi Set.
pub mod multiset;

// This module contains the implementation of Threshold Clock.
pub mod tclock;

// Top-level re-exports.
pub use crate::clock::{AEClock, ARClock, BEClock, Clock, VClock};
pub use crate::multiset::MultiSet;
pub use crate::set::AboveExSet;
pub use crate::set::AboveRangeSet;
pub use crate::set::BelowExSet;
pub use crate::set::MaxSet;
pub use crate::tclock::TClock;
pub use crate::traits::{subtract_iter, Actor, Count, EventSet};

// Tests
#[cfg(test)]
mod tests;
