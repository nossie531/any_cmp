//! Provider of [`AsAnyPartialEq`].

use crate::AnyPartialEq;

/// Support upcast to [`AnyPartialEq`].
pub trait AsAnyPartialEq: AnyPartialEq {
    /// Upcast `self` to [`AnyPartialEq`].
    #[must_use]
    fn as_any_partial_eq_ref(&self) -> &dyn AnyPartialEq;

    /// Upcast `self` to mutable [`AnyPartialEq`].
    #[must_use]
    fn as_any_partial_eq_mut(&mut self) -> &mut dyn AnyPartialEq;
}

impl<T> AsAnyPartialEq for T
where
    T: AnyPartialEq,
{
    #[inline(always)]
    fn as_any_partial_eq_ref(&self) -> &dyn AnyPartialEq {
        self
    }

    #[inline(always)]
    fn as_any_partial_eq_mut(&mut self) -> &mut dyn AnyPartialEq {
        self
    }
}
