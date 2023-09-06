//! Provider of [`AsAnyPartialOrd`].

use crate::AnyPartialOrd;

/// Support upcast to [`AnyPartialOrd`].
pub trait AsAnyPartialOrd: AnyPartialOrd {
    /// Upcast `self` to [`AnyPartialOrd`].
    fn as_any_partial_ord_ref(&self) -> &dyn AnyPartialOrd;

    /// Upcast `self` to mutable [`AnyPartialOrd`].
    fn as_any_partial_ord_mut(&mut self) -> &mut dyn AnyPartialOrd;
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
}
