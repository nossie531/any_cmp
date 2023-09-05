//! Provider of [`AsAnyEq`].

use crate::AnyEq;

/// Support upcast to [`AnyEq`].
pub trait AsAnyEq: AnyEq {
    /// Upcast `self` to [`AnyEq`].
    fn as_any_eq(&self) -> &dyn AnyEq;
}

impl<T> AsAnyEq for T
where
    T: AnyEq,
{
    #[inline(always)]
    fn as_any_eq(&self) -> &dyn AnyEq {
        self
    }
}
