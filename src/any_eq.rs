//! Provider of [`AnyEq`].

use super::any_partial_eq::AnyPartialEq;
use crate::upcast::AsAnyPartialEq;
use std::any::Any;
use std::fmt::{Debug, Formatter, Result};

/// Like [`Eq`], but this trait can be dynamic.
pub trait AnyEq: AnyPartialEq + AsAnyPartialEq {
    // NOP.
}

impl<T> AnyEq for T
where
    T: Any + Eq,
{
    // NOP.
}

impl PartialEq for dyn AnyEq {
    fn eq(&self, other: &Self) -> bool {
        self.any_eq(other.as_any_partial_eq_ref())
    }
}

impl PartialEq for dyn AnyEq + Send {
    fn eq(&self, other: &Self) -> bool {
        self.any_eq(other.as_any_partial_eq_ref())
    }
}

impl PartialEq for dyn AnyEq + Send + Sync {
    fn eq(&self, other: &Self) -> bool {
        self.any_eq(other.as_any_partial_eq_ref())
    }
}

impl Eq for dyn AnyEq {}

impl Eq for dyn AnyEq + Send {}

impl Eq for dyn AnyEq + Send + Sync {}

impl Debug for dyn AnyEq {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("AnyEq").finish_non_exhaustive()
    }
}

impl Debug for dyn AnyEq + Send {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("AnyEq").finish_non_exhaustive()
    }
}

impl Debug for dyn AnyEq + Send + Sync {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("AnyEq").finish_non_exhaustive()
    }
}
