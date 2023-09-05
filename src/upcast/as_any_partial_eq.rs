//! Provider of [`AsAnyPartialEq`].

use crate::AnyPartialEq;

/// Support upcast to [`AnyPartialEq`].
pub trait AsAnyPartialEq: AnyPartialEq {
    /// Upcast `self` to [`AnyPartialEq`].
    fn as_any_partial_eq(&self) -> &dyn AnyPartialEq;
}

impl<T> AsAnyPartialEq for T
where
    T: AnyPartialEq,
{
    #[inline(always)]
    fn as_any_partial_eq(&self) -> &dyn AnyPartialEq {
        self
    }
}
