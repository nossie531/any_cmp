//! Provider of [`AsAnyPartialOrd`].

use crate::AnyPartialOrd;

/// Support upcast to [`AnyPartialOrd`].
pub trait AsAnyPartialOrd: AnyPartialOrd {
    /// Upcast `self` to [`AnyPartialOrd`].
    #[must_use]
    fn as_any_partial_ord_ref(&self) -> &dyn AnyPartialOrd;

    /// Upcast `self` to mutable [`AnyPartialOrd`].
    #[must_use]
    fn as_any_partial_ord_mut(&mut self) -> &mut dyn AnyPartialOrd;

    /// Upcast `self` to boxed [`AnyPartialOrd`].
    #[must_use]
    fn as_any_partial_ord_box(self: Box<Self>) -> Box<dyn AnyPartialOrd>;
}

impl<T> AsAnyPartialOrd for T
where
    T: AnyPartialOrd,
{
    #[inline(always)]
    fn as_any_partial_ord_ref(&self) -> &dyn AnyPartialOrd {
        self
    }

    #[inline(always)]
    fn as_any_partial_ord_mut(&mut self) -> &mut dyn AnyPartialOrd {
        self
    }

    #[inline(always)]
    fn as_any_partial_ord_box(self: Box<Self>) -> Box<dyn AnyPartialOrd> {
        self
    }
}
