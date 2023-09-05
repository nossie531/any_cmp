//! Provider of [`AnyHash`].

use std::any::Any;
use std::fmt::{Debug, Formatter, Result};
use std::hash::{Hash, Hasher};

/// Like [`Hash`], but this trait can be dynamic.
pub trait AnyHash {
    /// Feeds this value into the given [`Hasher`].
    fn any_hash(&self, state: &mut dyn Hasher);
}

impl<T> AnyHash for T
where
    T: Any + Hash,
{
    fn any_hash(&self, state: &mut dyn Hasher) {
        Hash::hash(&(self.type_id(), self), &mut Box::new(state));
    }
}

impl Hash for dyn AnyHash {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.any_hash(state);
    }
}

impl Hash for dyn AnyHash + Send {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.any_hash(state);
    }
}

impl Hash for dyn AnyHash + Send + Sync {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.any_hash(state);
    }
}

impl Debug for dyn AnyHash {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("AnyHash").finish_non_exhaustive()
    }
}

impl Debug for dyn AnyHash + Send {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("AnyHash").finish_non_exhaustive()
    }
}

impl Debug for dyn AnyHash + Send + Sync {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("AnyHash").finish_non_exhaustive()
    }
}
