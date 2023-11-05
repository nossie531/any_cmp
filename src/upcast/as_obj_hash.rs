//! Provider of [`AsObjHash`].

use crate::ObjHash;

/// Support upcast to [`ObjHash`].
pub trait AsObjHash: ObjHash {
    /// Upcast `self` to [`ObjHash`].
    #[must_use]
    fn as_obj_hash_ref(&self) -> &dyn ObjHash;

    /// Upcast `self` to mutable [`ObjHash`].
    #[must_use]
    fn as_obj_hash_mut(&mut self) -> &mut dyn ObjHash;

    /// Upcast `self` to boxed [`ObjHash`].
    #[must_use]
    fn as_obj_hash_box(self: Box<Self>) -> Box<dyn ObjHash>;
}

impl<T> AsObjHash for T
where
    T: ObjHash,
{
    #[inline(always)]
    fn as_obj_hash_ref(&self) -> &dyn ObjHash {
        self
    }

    #[inline(always)]
    fn as_obj_hash_mut(&mut self) -> &mut dyn ObjHash {
        self
    }

    #[inline(always)]
    fn as_obj_hash_box(self: Box<Self>) -> Box<dyn ObjHash> {
        self
    }
}
