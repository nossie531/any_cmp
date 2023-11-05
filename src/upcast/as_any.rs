//! Provider of [`AsAny`].

use std::any::Any;

/// Support upcast to [`Any`].
pub trait AsAny: Any {
    /// Upcast `self` to [`Any`].
    #[must_use]
    fn as_any_ref(&self) -> &dyn Any;

    /// Upcast `self` to mutable [`Any`].
    #[must_use]
    fn as_any_mut(&mut self) -> &mut dyn Any;

    /// Upcast `self` to boxed [`Any`].
    #[must_use]
    fn as_any_box(self: Box<Self>) -> Box<dyn Any>;
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

    #[inline(always)]
    fn as_any_box(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}
