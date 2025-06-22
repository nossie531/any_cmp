//! Provider of [`ObjHash`].

use crate::prelude::*;
use std::fmt::{Debug, Formatter, Result};
use std::hash::{Hash, Hasher};

/// Trait that combines [`AnyEq`] and [`AnyHash`].
pub trait ObjHash: AnyEq + AnyHash {}

impl<T> ObjHash for T where T: AnyEq + AnyHash {}

impl PartialEq for dyn ObjHash {
    fn eq(&self, other: &Self) -> bool {
        self.any_eq(other as &dyn AnyPartialEq)
    }
}

impl PartialEq for dyn ObjHash + Send {
    fn eq(&self, other: &Self) -> bool {
        self.any_eq(other as &dyn AnyPartialEq)
    }
}

impl PartialEq for dyn ObjHash + Send + Sync {
    fn eq(&self, other: &Self) -> bool {
        self.any_eq(other as &dyn AnyPartialEq)
    }
}

impl Eq for dyn ObjHash {}

impl Eq for dyn ObjHash + Send {}

impl Eq for dyn ObjHash + Send + Sync {}

impl Hash for dyn ObjHash {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.any_hash(state);
    }
}

impl Hash for dyn ObjHash + Send {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.any_hash(state);
    }
}

impl Hash for dyn ObjHash + Send + Sync {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.any_hash(state);
    }
}

impl Debug for dyn ObjHash {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("ObjHash").finish_non_exhaustive()
    }
}

impl Debug for dyn ObjHash + Send {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("ObjHash").finish_non_exhaustive()
    }
}

impl Debug for dyn ObjHash + Send + Sync {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("ObjHash").finish_non_exhaustive()
    }
}
