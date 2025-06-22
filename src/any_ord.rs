//! Provider of [`AnyOrd`].

use crate::prelude::*;
use std::any::Any;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter, Result};

/// Like [`Ord`], but this trait can be dynamic.
///
/// In a cross-type comparison, the [`TypeId`] of the type is compared first.
/// And since the order of the [`TypeId`] depends on the Rust version, the
/// comparison result depends on it as well.
///
/// [`TypeId`]: core::any::TypeId
pub trait AnyOrd: AnyPartialOrd {
    /// This method returns an ordering between `self` and `other`.
    #[must_use]
    fn any_cmp(&self, other: &dyn AnyOrd) -> Ordering;
}

impl<T> AnyOrd for T
where
    T: Any + Ord,
{
    fn any_cmp(&self, other: &dyn AnyOrd) -> Ordering {
        self.any_partial_cmp(other as &dyn AnyPartialOrd).unwrap()
    }
}

impl PartialEq for dyn AnyOrd {
    fn eq(&self, other: &Self) -> bool {
        self.any_eq(other as &dyn AnyPartialEq)
    }
}

impl PartialEq for dyn AnyOrd + Send {
    fn eq(&self, other: &Self) -> bool {
        self.any_eq(other as &dyn AnyPartialEq)
    }
}

impl PartialEq for dyn AnyOrd + Send + Sync {
    fn eq(&self, other: &Self) -> bool {
        self.any_eq(other as &dyn AnyPartialEq)
    }
}

impl Eq for dyn AnyOrd {}

impl Eq for dyn AnyOrd + Send {}

impl Eq for dyn AnyOrd + Send + Sync {}

impl PartialOrd for dyn AnyOrd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.any_partial_cmp(other as &dyn AnyPartialOrd)
    }
}

impl PartialOrd for dyn AnyOrd + Send {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.any_partial_cmp(other as &dyn AnyPartialOrd)
    }
}

impl PartialOrd for dyn AnyOrd + Send + Sync {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.any_partial_cmp(other as &dyn AnyPartialOrd)
    }
}

impl Ord for dyn AnyOrd {
    fn cmp(&self, other: &Self) -> Ordering {
        self.any_cmp(other)
    }
}

impl Ord for dyn AnyOrd + Send {
    fn cmp(&self, other: &Self) -> Ordering {
        self.any_cmp(other)
    }
}

impl Ord for dyn AnyOrd + Send + Sync {
    fn cmp(&self, other: &Self) -> Ordering {
        self.any_cmp(other)
    }
}

impl Debug for dyn AnyOrd {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("AnyOrd").finish_non_exhaustive()
    }
}

impl Debug for dyn AnyOrd + Send {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("AnyOrd").finish_non_exhaustive()
    }
}

impl Debug for dyn AnyOrd + Send + Sync {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("AnyOrd").finish_non_exhaustive()
    }
}
