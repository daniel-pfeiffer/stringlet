//! Workaround for `[u8; SIZE + Kind::EXTRA_LEN]`, as long as “generic parameters may not be used in const operations”

use crate::*;

use core::{slice::from_raw_parts, slice::from_raw_parts_mut};

impl<Kind: StringletKind, const SIZE: usize> StringletBase<Kind, SIZE> {
    #[inline]
    /// Workaround to get the extra len byte for `VarStringlet`.
    pub(crate) const fn var_last(&self) -> u8 {
        debug_assert!(Kind::VAR, "unchecked call");
        // SAFETY: 1 byte after SIZE only used for VarStringlet and always initialized
        unsafe { (self as *const Self as *const u8).add(SIZE).read() }
    }

    #[inline]
    /// Workaround for `&[u8; SIZE + Kind::EXTRA_LEN]`.
    pub(crate) const fn as_slice(&self) -> &[u8] {
        let ptr = self as *const Self as *const u8;
        unsafe { from_raw_parts(ptr, SIZE + size_of::<Kind::ExtraLen>()) }
    }

    #[inline]
    pub(crate) const fn _as_slice_mut(&mut self) -> &mut [u8] {
        let ptr = self as *mut Self as *mut u8;
        unsafe { from_raw_parts_mut(ptr, SIZE + size_of::<Kind::ExtraLen>()) }
    }

    /* #[inline]
    pub(crate) fn into_parts(self) -> ([u8; SIZE], [u8; B]) {
        (self.0, self.1)
    }

    // If this becomes stabilised before genereral const generics, consider switching:
    #[inline]
    pub(crate) fn as_array(&self) -> &[u8; SIZE + B] {
        unsafe { &*(self as *const Self as *const _) }
    }

    #[inline]
    pub(crate) fn as_array_mut(&mut self) -> &mut [u8; SIZE + B] {
        unsafe { &mut *(self as *mut Self as *mut _) }
    }

    #[inline]
    pub(crate) fn into_array(self) -> [T; SIZE + B] {
        let this = ManuallyDrop::new(self);
        unsafe { transmute_copy(&*this as &Self) }
    } */
}
