//! Provider of [`AsAnyEq`].

use crate::AnyEq;

/// Support upcast to [`AnyEq`].
pub trait AsAnyEq: AnyEq {
    /// Upcast `self` to [`AnyEq`].
    fn as_any_eq_ref(&self) -> &dyn AnyEq;

    /// Upcast `self` to mutable [`AnyEq`].
    fn as_any_eq_mut(&mut self) -> &mut dyn AnyEq;
}

impl<T> AsAnyEq for T
where
    T: AnyEq,
{
    #[inline(always)]
    fn as_any_eq_ref(&self) -> &dyn AnyEq {
        self
    }

    #[inline(always)]
    fn as_any_eq_mut(&mut self) -> &mut dyn AnyEq {
        self
    }
}
