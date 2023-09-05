//! Provider of [`AsObjHash`].

use crate::ObjHash;

/// Support upcast to [`ObjHash`].
pub trait AsObjHash: ObjHash {
    /// Upcast `self` to [`ObjHash`].
    fn as_obj_hash(&self) -> &dyn ObjHash;
}

impl<T> AsObjHash for T
where
    T: ObjHash,
{
    #[inline(always)]
    fn as_obj_hash(&self) -> &dyn ObjHash {
        self
    }
}
