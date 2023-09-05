//! Provider of [`AsAnyOrd`].

use crate::AnyOrd;

/// Support upcast to [`AnyOrd`].
pub trait AsAnyOrd: AnyOrd {
    /// Upcast `self` to [`AnyOrd`].
    fn as_any_ord(&self) -> &dyn AnyOrd;
}

impl<T> AsAnyOrd for T
where
    T: AnyOrd,
{
    #[inline(always)]
    fn as_any_ord(&self) -> &dyn AnyOrd {
        self
    }
}
