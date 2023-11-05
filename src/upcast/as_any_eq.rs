//! Provider of [`AsAnyEq`].

use crate::AnyEq;

/// Support upcast to [`AnyEq`].
pub trait AsAnyEq: AnyEq {
    /// Upcast `self` to [`AnyEq`].
    #[must_use]
    fn as_any_eq_ref(&self) -> &dyn AnyEq;

    /// Upcast `self` to mutable [`AnyEq`].
    #[must_use]
    fn as_any_eq_mut(&mut self) -> &mut dyn AnyEq;

    /// Upcast `self` to boxed [`AnyEq`].
    #[must_use]
    fn as_any_eq_box(self: Box<Self>) -> Box<dyn AnyEq>;
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

    #[inline(always)]
    fn as_any_eq_box(self: Box<Self>) -> Box<dyn AnyEq> {
        self
    }
}
