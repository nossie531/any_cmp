//! Support dynamic type comparisons.
//!
//! *The author of this crate is not good at English.*
//! *Forgive me if the document is hard to read.*

#![warn(missing_docs)]

pub mod prelude;

mod any_eq;
mod any_hash;
mod any_ord;
mod any_partial_eq;
mod any_partial_ord;
mod obj_hash;

pub use any_eq::*;
pub use any_hash::*;
pub use any_ord::*;
pub use any_partial_eq::*;
pub use any_partial_ord::*;
pub use obj_hash::*;
