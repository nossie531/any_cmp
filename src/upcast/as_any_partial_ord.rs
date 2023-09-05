//! Provider of [`AsAnyPartialOrd`].

use crate::AnyPartialOrd;

/// Support upcast to [`AnyPartialOrd`].
pub trait AsAnyPartialOrd: AnyPartialOrd {
    /// Upcast `self` to [`AnyPartialOrd`].
    fn as_any_partial_ord(&self) -> &dyn AnyPartialOrd;
}

impl<T> AsAnyPartialOrd for T
where
    T: AnyPartialOrd,
{
    #[inline(always)]
    fn as_any_partial_ord(&self) -> &dyn AnyPartialOrd {
        self
    }
}
