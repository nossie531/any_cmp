//! Provider of [`AnyPartialEq`].

use crate::upcast::AsAny;
use std::any::Any;
use std::fmt::{Debug, Formatter, Result};

/// Like [`PartialEq`], but this trait can be dynamic.
pub trait AnyPartialEq: Any + AsAny {
    /// This method tests for `self` and `other` values to be equal.
    #[must_use]
    fn any_eq(&self, other: &dyn AnyPartialEq) -> bool;

    /// This method tests for `self` and `other` values to be not equal.
    /// The default implementation is almost always sufficient, and should
    /// not be overridden without very good reason.
    #[must_use]
    fn any_ne(&self, other: &dyn AnyPartialEq) -> bool {
        !self.any_eq(other)
    }
}

impl<T> AnyPartialEq for T
where
    T: Any + PartialEq,
{
    fn any_eq(&self, other: &dyn AnyPartialEq) -> bool {
        if self.type_id() != other.type_id() {
            return false;
        }

        self.eq(other.as_any_ref().downcast_ref().unwrap())
    }
}

impl PartialEq for dyn AnyPartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.any_eq(other)
    }
}

impl PartialEq for dyn AnyPartialEq + Send {
    fn eq(&self, other: &Self) -> bool {
        self.any_eq(other)
    }
}

impl PartialEq for dyn AnyPartialEq + Send + Sync {
    fn eq(&self, other: &Self) -> bool {
        self.any_eq(other)
    }
}

impl Debug for dyn AnyPartialEq {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("AnyPartialEq").finish_non_exhaustive()
    }
}

impl Debug for dyn AnyPartialEq + Send {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("AnyPartialEq").finish_non_exhaustive()
    }
}

impl Debug for dyn AnyPartialEq + Send + Sync {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("AnyPartialEq").finish_non_exhaustive()
    }
}
