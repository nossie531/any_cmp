//! Provider of [`AsAny`].

use std::any::Any;

/// Support upcast to [`Any`].
pub trait AsAny: Any {
    /// Upcast `self` to [`Any`].
    fn as_any(&self) -> &dyn Any;
}

impl<T> AsAny for T
where
    T: Any,
{
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
}
