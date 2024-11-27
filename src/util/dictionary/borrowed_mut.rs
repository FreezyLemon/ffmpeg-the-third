use core::fmt;
use std::marker::PhantomData;

use crate::ffi::*;

pub struct DictionaryMut<'d> {
    ptr: *mut AVDictionary,
    _marker: PhantomData<&'d mut AVDictionary>,
}

impl<'d> DictionaryMut<'d> {
    pub unsafe fn from_raw(ptr: *mut AVDictionary) -> Self {
        DictionaryMut {
            ptr,
            _marker: PhantomData,
        }
    }

    pub fn as_ptr(&self) -> *const AVDictionary {
        self.ptr
    }

    pub fn as_mut_ptr(&mut self) -> *mut AVDictionary {
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
