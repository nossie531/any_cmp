//! Support dynamic type comparisons.

mod any_eq;
mod any_hash;
mod any_ord;
mod any_partial_eq;
mod any_partial_ord;
mod obj_hash;
pub mod upcast;

pub use any_eq::AnyEq;
pub use any_hash::AnyHash;
pub use any_ord::AnyOrd;
pub use any_partial_eq::AnyPartialEq;
pub use any_partial_ord::AnyPartialOrd;
pub use obj_hash::ObjHash;
