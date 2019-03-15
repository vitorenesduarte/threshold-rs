// This module contains the definition of `Count`, `Actor` and `EventSet`
// traits.
mod traits;

// This module contains implementations of quickcheck::Arbitrary trait.
mod arbitrary;

// This module contains the implementation of a Max Set.
pub mod maxset;

// This module contains the implementation of a Below-Exception Set.
pub mod below_exset;

// This module contains the implementation of a Clock.
pub mod clock;

// This module contains the implementation of a Multi Set.
pub mod multiset;

// This module contains the implementation of Threshold Clock.
pub mod tclock;

// Top-level re-exports.
pub use crate::below_exset::BelowExSet;
pub use crate::clock::{Clock, Dot, VClock};
pub use crate::maxset::MaxSet;
pub use crate::multiset::MultiSet;
pub use crate::tclock::TClock;
pub use crate::traits::{Actor, Count, EventSet};
