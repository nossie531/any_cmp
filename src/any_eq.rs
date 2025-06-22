//! Provider of [`AnyEq`].

use crate::prelude::*;
use std::any::Any;
use std::fmt::{Debug, Formatter, Result};

/// Like [`Eq`], but this trait can be dynamic.
pub trait AnyEq: AnyPartialEq {
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
        self.any_eq(other as &dyn AnyPartialEq)
    }
}

impl PartialEq for dyn AnyEq + Send {
    fn eq(&self, other: &Self) -> bool {
        self.any_eq(other as &dyn AnyPartialEq)
    }
}

impl PartialEq for dyn AnyEq + Send + Sync {
    fn eq(&self, other: &Self) -> bool {
        self.any_eq(other as &dyn AnyPartialEq)
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
