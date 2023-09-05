/*! Support crait upcast.

# Notes

In the future, when [trait_upcasting] is released,
this module will not be needed as much.

[trait_upcasting]:
    https://doc.rust-lang.org/beta/unstable-book/language-features/trait-upcasting.html
*/

mod as_any;
mod as_any_eq;
mod as_any_hash;
mod as_any_ord;
mod as_any_partial_eq;
mod as_any_partial_ord;
mod as_obj_hash;

pub use as_any::AsAny;
pub use as_any_eq::AsAnyEq;
pub use as_any_hash::AsAnyHash;
pub use as_any_ord::AsAnyOrd;
pub use as_any_partial_eq::AsAnyPartialEq;
pub use as_any_partial_ord::AsAnyPartialOrd;
pub use as_obj_hash::AsObjHash;
