//! Provider of [`AsAnyHash`].

use crate::AnyHash;

/// Support upcast to [`AnyHash`].
pub trait AsAnyHash: AnyHash {
    /// Upcast `self` to [`AnyHash`].
    #[must_use]
    fn as_any_hash_ref(&self) -> &dyn AnyHash;

    /// Upcast `self` to mutable [`AnyHash`].
    #[must_use]
    fn as_any_hash_mut(&mut self) -> &mut dyn AnyHash;

    /// Upcast `self` to boxed [`AnyHash`].
    #[must_use]
    fn as_any_hash_box(self: Box<Self>) -> Box<dyn AnyHash>;
}

impl<T> AsAnyHash for T
where
    T: AnyHash,
{
    #[inline(always)]
    fn as_any_hash_ref(&self) -> &dyn AnyHash {
        self
    }

    #[inline(always)]
    fn as_any_hash_mut(&mut self) -> &mut dyn AnyHash {
        self
    }

    #[inline(always)]
    fn as_any_hash_box(self: Box<Self>) -> Box<dyn AnyHash> {
        self
    }
}
