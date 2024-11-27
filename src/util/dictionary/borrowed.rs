use std::fmt;
use std::marker::PhantomData;

use super::{impls, Dictionary, Flags, Iter};
use crate::ffi::*;

pub struct DictionaryRef<'d> {
    ptr: *const AVDictionary,
    _marker: PhantomData<&'d AVDictionary>,
}

impl<'d> DictionaryRef<'d> {
    pub unsafe fn from_raw(ptr: *const AVDictionary) -> Self {
        DictionaryRef {
            ptr,
            _marker: PhantomData,
        }
    }

    pub fn as_ptr(&self) -> *const AVDictionary {
        self.ptr
    }
}

impl<'d> DictionaryRef<'d> {
    pub fn get(&self, key: &str) -> Option<&'d str> {
        unsafe { impls::get(self.as_ptr(), key, Flags::empty()) }
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self.as_ptr())
    }

    pub fn to_owned(&self) -> Dictionary {
        self.iter().collect()
    }
}

impl<'a> IntoIterator for &'a DictionaryRef<'a> {
    type Item = (&'a str, &'a str);
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> fmt::Debug for DictionaryRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}
