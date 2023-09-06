//! Provider of [`AsAnyOrd`].

use crate::AnyOrd;

/// Support upcast to [`AnyOrd`].
pub trait AsAnyOrd: AnyOrd {
    /// Upcast `self` to [`AnyOrd`].
    fn as_any_ord_ref(&self) -> &dyn AnyOrd;

    /// Upcast `self` to mutable [`AnyOrd`].
    fn as_any_ord_mut(&mut self) -> &mut dyn AnyOrd;
}

impl<T> AsAnyOrd for T
where
    T: AnyOrd,
{
    #[inline(always)]
    fn as_any_ord_ref(&self) -> &dyn AnyOrd {
        self
    }

    #[inline(always)]
    fn as_any_ord_mut(&mut self) -> &mut dyn AnyOrd {
        self
    }
}
