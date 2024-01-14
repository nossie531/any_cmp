/*! Support trait upcast.

# Notes

In the future, when [`trait_upcasting`] is released,
this module will not be needed as much.

[`trait_upcasting`]:
    https://doc.rust-lang.org/beta/unstable-book/language-features/trait-upcasting.html
*/

mod as_any;
mod as_any_eq;
mod as_any_hash;
mod as_any_ord;
mod as_any_partial_eq;
mod as_any_partial_ord;
mod as_obj_hash;

pub use as_any::*;
pub use as_any_eq::*;
pub use as_any_hash::*;
pub use as_any_ord::*;
pub use as_any_partial_eq::*;
pub use as_any_partial_ord::*;
pub use as_obj_hash::*;
