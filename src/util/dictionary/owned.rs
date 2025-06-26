use std::fmt;
use std::ptr;

use libc::c_int;

use crate::ffi::*;

use super::flag::Flags;
use super::{DictionaryMut, DictionaryRef};

pub struct Dictionary {
    ptr: *mut AVDictionary,
}

impl Default for Dictionary {
    fn default() -> Self {
        Self::new()
    }
}

impl Dictionary {
    pub unsafe fn from_raw(ptr: *mut AVDictionary) -> Self {
        Self { ptr }
    }

    pub fn into_raw(mut self) -> *mut AVDictionary {
        let result = self.ptr;
        self.ptr = ptr::null_mut();

        result
    }

    pub fn replace_with(&mut self, new: Self) {
        unsafe { av_dict_free(&mut self.ptr) };
        self.ptr = new.into_raw();
    }

    pub fn as_ptr(&self) -> *const AVDictionary {
        self.ptr
    }

    pub fn as_mut_ptr(&mut self) -> &mut *mut AVDictionary {
        &mut self.ptr
    }

    pub fn as_ref(&self) -> DictionaryRef {
        unsafe { DictionaryRef::from_raw(self.as_ptr()) }
    }

    pub fn as_mut(&mut self) -> DictionaryMut {
        unsafe { DictionaryMut::from_raw(self.as_mut_ptr()) }
    }
}

impl Dictionary {
    pub fn new() -> Self {
        Self {
            ptr: ptr::null_mut(),
        }
    }
}

impl<S, U> FromIterator<(S, U)> for Dictionary
where
    S: AsRef<str>,
    U: AsRef<str>,
{
    fn from_iter<T: IntoIterator<Item = (S, U)>>(iter: T) -> Self {
        let mut result = Dictionary::new();

        for (key, value) in iter {
            result.set(key.as_ref(), value.as_ref())
        }

        result
    }
}

impl Clone for Dictionary {
    fn clone(&self) -> Self {
        let mut dictionary = Dictionary::new();
        dictionary.clone_from(self);

        dictionary
    }

    fn clone_from(&mut self, source: &Self) {
        unsafe {
            av_dict_copy(
                &mut self.ptr,
                source.as_ptr(),
                Flags::MULTIKEY.bits() as c_int,
            );
        }
    }
}

impl Drop for Dictionary {
    fn drop(&mut self) {
        unsafe {
            av_dict_free(&mut self.ptr);
        }
    }
}

impl fmt::Debug for Dictionary {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.as_ref().fmt(fmt)
    }
}
