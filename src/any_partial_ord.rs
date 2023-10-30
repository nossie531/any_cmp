//! Provider of [`AnyPartialOrd`].

use super::AnyPartialEq;
use crate::upcast::AsAnyPartialEq;
use std::any::Any;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter, Result};

/// Like [`PartialOrd`], but this trait can be dynamic.
///
/// In a cross-type comparison, the [`TypeId`] of the type is compared first.
/// And since the order of the [`TypeId`] depends on the Rust version, the
/// comparison result also depends on it.
///
/// [`TypeId`]: core::any::TypeId
pub trait AnyPartialOrd: AnyPartialEq + AsAnyPartialEq {
    /// This method returns an ordering between `self` and `other` values if one exists.
    #[must_use]
    fn any_partial_cmp(&self, other: &dyn AnyPartialOrd) -> Option<Ordering>;
}

impl<T> AnyPartialOrd for T
where
    T: Any + PartialOrd,
{
    fn any_partial_cmp(&self, other: &dyn AnyPartialOrd) -> Option<Ordering> {
        if self.type_id() != other.type_id() {
            return Some(self.type_id().cmp(&other.type_id()));
        }

        self.partial_cmp(other.as_any_ref().downcast_ref().unwrap())
    }
}

impl PartialEq for dyn AnyPartialOrd {
    fn eq(&self, other: &Self) -> bool {
        self.any_eq(other.as_any_partial_eq_ref())
    }
}

impl PartialEq for dyn AnyPartialOrd + Send {
    fn eq(&self, other: &Self) -> bool {
        self.any_eq(other.as_any_partial_eq_ref())
    }
}

impl PartialEq for dyn AnyPartialOrd + Send + Sync {
    fn eq(&self, other: &Self) -> bool {
        self.any_eq(other.as_any_partial_eq_ref())
    }
}

impl PartialOrd for dyn AnyPartialOrd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.any_partial_cmp(other)
    }
}

impl PartialOrd for dyn AnyPartialOrd + Send {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.any_partial_cmp(other)
    }
}

impl PartialOrd for dyn AnyPartialOrd + Send + Sync {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.any_partial_cmp(other)
    }
}

impl Debug for dyn AnyPartialOrd {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("AnyPartialOrd").finish_non_exhaustive()
    }
}

impl Debug for dyn AnyPartialOrd + Send {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("AnyPartialOrd").finish_non_exhaustive()
    }
}

impl Debug for dyn AnyPartialOrd + Send + Sync {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("AnyPartialOrd").finish_non_exhaustive()
    }
}
