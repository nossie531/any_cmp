//! Provider of [`AsAnyHash`].

use crate::AnyHash;

/// Support upcast to [`AnyHash`].
pub trait AsAnyHash: AnyHash {
    /// Upcast `self` to [`AnyHash`].
    fn as_any_hash(&self) -> &dyn AnyHash;
}

impl<T> AsAnyHash for T
where
    T: AnyHash,
{
    #[inline(always)]
    fn as_any_hash(&self) -> &dyn AnyHash {
        self
    }
}
