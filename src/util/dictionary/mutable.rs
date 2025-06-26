use std::fmt;

use super::Dictionary;
use crate::ffi::*;

pub struct DictionaryMut<'d> {
    ptr: &'d mut *mut AVDictionary,
}

impl<'d> DictionaryMut<'d> {
    pub unsafe fn from_raw(ptr: &'d mut *mut AVDictionary) -> Self {
        DictionaryMut { ptr }
    }

    pub fn replace_with(&mut self, new: Dictionary) {
        unsafe { av_dict_free(self.ptr) };
        *self.ptr = new.into_raw();
    }

    pub fn as_ptr(&self) -> *const AVDictionary {
        *self.ptr
    }

    pub fn as_mut_ptr(&mut self) -> &mut *mut AVDictionary {
        self.ptr
    }

    pub fn as_ref(&self) -> super::DictionaryRef<'d> {
        unsafe { super::DictionaryRef::from_raw(self.as_ptr()) }
    }
}

impl<'d> fmt::Debug for DictionaryMut<'d> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}
