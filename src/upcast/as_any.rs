//! Provider of [`AsAny`].

use std::any::Any;

/// Support upcast to [`Any`].
pub trait AsAny: Any {
    /// Upcast `self` to [`Any`].
    fn as_any_ref(&self) -> &dyn Any;

    /// Upcast `self` to mutable [`Any`].
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T> AsAny for T
where
    T: Any,
{
    #[inline(always)]
    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    #[inline(always)]
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
