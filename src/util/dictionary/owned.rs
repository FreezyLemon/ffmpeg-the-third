use std::fmt;
use std::ptr;

use libc::c_int;

use crate::ffi::*;

use super::{DictionaryRef, DictionaryMut};
use super::flag::Flags;
use super::impls;

pub struct Dictionary {
    ptr: *mut AVDictionary,
}

impl Default for Dictionary {
    fn default() -> Self {
        Self::new()
    }
}

impl Dictionary {
    pub unsafe fn own(ptr: *mut AVDictionary) -> Self {
        Self { ptr }
    }

    pub unsafe fn disown(mut self) -> *mut AVDictionary {
        let result = self.ptr;
        self.ptr = ptr::null_mut();

        result
    }

    pub fn as_ptr(&self) -> *const AVDictionary {
        self.ptr
    }

    pub fn as_mut_ptr(&mut self) -> *mut AVDictionary {
        self.ptr
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

    pub fn set(&mut self, key: &str, value: &str) {
        impls::set(&mut self.ptr, key, value, Flags::empty())
    }

    pub fn set_with_flags(&mut self, key: &str, value: &str, flags: Flags) {
        impls::set(&mut self.ptr, key, value, flags)
    }

    pub fn get<'d>(&'d self, key: &str) -> Option<&'d str> {
        // SAFETY: Returned lifetime is bounded by borrow on self
        unsafe { impls::get(self.as_ptr(), key, Flags::empty()) }
    }

    pub fn get_with_flags<'d>(&'d self, key: &str, flags: Flags) -> Option<&'d str> {
        // SAFETY: Returned lifetime is bounded by borrow on self
        unsafe { impls::get(self.as_ptr(), key, flags) }
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
