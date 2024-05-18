use std::ffi::{CStr, CString};
use std::marker::PhantomData;

use libc::c_int;

use crate::ffi::*;

// Important note:
// The header libavutil/dict.h defines `AVDictionary` to be a zero-sized struct, while
// the implementation in libavutil/dict.c privately has a different struct definition.
// This means that the generated `AVDictionary` is a ZST which is *wrong*, but exactly
// what's intended by FFmpeg. I am not sure if dereferencing a pointer to this ZST is UB
// if the pointer is otherwise valid (but for a different struct), so:
//
// **Take care to never dereference any *const AVDictionary or *mut AVDictionary in here!**

#[derive(Debug)]
pub enum Dictionary<'dict> {
    Owned(DictionaryOwned),
    Borrowed(DictionaryBorrowed<'dict>),
}

// private structs
#[derive(Debug)]
struct DictionaryOwned {
    ptr: *mut AVDictionary,
    _marker: PhantomData<Option<AVDictionary>>,
}

#[derive(Debug, Clone)]
struct DictionaryBorrowed<'dict> {
    ptr: *const AVDictionary,
    _marker: PhantomData<Option<&'dict AVDictionary>>,
}

/// A read-only dictionary entry. This type is returned by <TODO>
#[derive(Debug, Clone, Copy)]
pub struct DictionaryEntry<'e> {
    av_entry: &'e AVDictionaryEntry,
}

impl<'e> DictionaryEntry<'e> {
    // We will assume that the key and value in the entry are valid and NUL-terminated because
    // this should be enforced by FFmpeg.
    // We will *not* assume that they are valid UTF-8, because c_char happily accepts values > 127
    // and av_dict_set is user-accessible. 

    pub fn key_raw(&self) -> &CStr {
        unsafe {
            CStr::from_ptr(self.av_entry.key)
        }
    }
    
    pub fn value_raw(&self) -> &CStr {
        unsafe {
            CStr::from_ptr(self.av_entry.value)
        }
    }

    pub fn key(&self) -> Option<&str> {
        self.key_raw().to_str().ok()
    }

    pub fn value(&self) -> Option<&str> {
        self.value_raw().to_str().ok()
    }
}

impl<'dict> Dictionary<'dict> {
    pub fn empty() -> Self {
        // SAFETY: Calling owned with a null pointer is valid
        unsafe {
            Self::owned(std::ptr::null_mut())
        }
    }

    // SAFETY: Make sure ptr is null or is a *const AVDictionary pointer as returned by
    //         the public FFmpeg API.
    // SAFETY: Make sure you really have ownership of the AVDictionary.
    unsafe fn owned(ptr: *mut AVDictionary) -> Self {
        Self::Owned(DictionaryOwned {
            ptr,
            _marker: PhantomData,
        })
    }

    pub fn borrowed<'s>(&'s self) -> Dictionary<'s> 
    where 'dict: 's
    {
        let ptr = match self {
            Dictionary::Owned(dict) => dict.ptr as *const _,
            Dictionary::Borrowed(dict) => dict.ptr,
        };

        Dictionary::Borrowed(DictionaryBorrowed {
            ptr,
            _marker: PhantomData,
        })
    }

    pub fn as_ptr(&self) -> *const AVDictionary {
        match self {
            Self::Owned(d) => d.ptr,
            Self::Borrowed(d) => d.ptr,
        }
    }

    // pub fn as_mut_ptr(&mut self) -> *mut AVDictionary {
    //     match &mut self.0 {
    //         Some(cow) => cow.to_mut(),
    //         None => std::ptr::null_mut(),
    //     }
    // }

    fn get<'entry>(&self, key: &CStr, prev: Option<DictionaryEntry>, flags: c_int) -> Result<DictionaryEntry<'entry>, DictGetError>
    where 'dict: 'entry
    {
        let prev: *const AVDictionaryEntry = match prev {
            Some(entry) => entry.av_entry,
            None => std::ptr::null(),
        };

        unsafe {
            // SAFETY: `key.as_ptr()` will return a valid non-null pointer.
            // SAFETY: `prev` is guaranteed to be null or made from a reference (= assumed to be valid).
            let av_entry_ptr = av_dict_get(
                self.as_ptr(),
                key.as_ptr(),
                prev,
                flags,
            );

            // SAFETY: We assume the returned pointer is null or a valid *const AVDictionaryEntry.
            // SAFETY: We also ensure that the arbitrary lifetime returned by as_ref is bounded by
            //         the lifetime of the containing AVDictionary (see function trait bounds).
            match av_entry_ptr.as_ref::<'entry>() {
                Some(av_entry) => Ok(DictionaryEntry { av_entry }),
                None => Err(DictGetError::NotFound),
            }
        }
    }

    // Note: Pass either a string type (String, &str), or a &[u8] without a NUL byte.
    pub fn get_first<K: Into<Vec<u8>>>(&self, key: K) -> Result<DictionaryEntry, DictGetError> 
    {
        let key = match CString::new(key) {
            Ok(c_string) => c_string,
            Err(_) => return Err(DictGetError::InvalidKey),
        };

        self.get(&key, None, 0)
    }

    pub fn get_many<K: Into<Vec<u8>>>(&self, key: K) -> Result<DictionaryGetManyIter, DictGetError>
    {
        let key = match CString::new(key) {
            Ok(c_string) => c_string,
            Err(_) => return Err(DictGetError::InvalidKey),
        };

        Ok(DictionaryGetManyIter::new(self.borrowed(), key, 0))
    }

    // pub fn get<K: Into<Vec<u8>>>(&self, key: K, )
}

pub struct DictionaryGetManyIter<'d: 'e, 'e>
{
    dict: Dictionary<'d>,
    key: CString,
    flags: c_int,
    prev: Option<DictionaryEntry<'e>>,
}

impl<'d: 'e, 'e> DictionaryGetManyIter<'d, 'e> {
    pub fn new(dict: Dictionary<'d>, key: CString, flags: c_int) -> Self {
        Self {
            dict,
            key,
            flags,
            prev: None,
        }
    }
}

impl<'d: 'e, 'e> Iterator for DictionaryGetManyIter<'d, 'e> {
    type Item = DictionaryEntry<'e>;

    fn next(&mut self) -> Option<Self::Item>
    {
        match self.dict.get(&self.key, self.prev, self.flags) {
            Ok(entry) => {
                self.prev = Some(entry);
                Some(entry)
            }
            Err(_) => None,
        }
    }
}

pub enum DictGetError {
    NotFound,
    InvalidKey,
}

impl<'d> Clone for Dictionary<'d> {
    fn clone(&self) -> Self {
        let mut dst = Self::empty();
        dst.clone_from(self);
        dst
    }
    
    fn clone_from(&mut self, source: &Self) {
        let mut dst_ptr = match self {
            Dictionary::Owned(dict) => dict.ptr,
            Dictionary::Borrowed(_) => std::ptr::null_mut(),
        };

        unsafe {
            // AV_DICT_MULTIKEY so that keys that exist multiple times will be kept.
            let copy_res = av_dict_copy(&mut dst_ptr, source.as_ptr(), AV_DICT_MULTIKEY);
            match self {
                Dictionary::Owned(ref mut dict) => dict.ptr = dst_ptr,
                Dictionary::Borrowed(_) => *self = Self::owned(dst_ptr),
            }

            // panic! after updating self so that the new memory is dropped correctly.
            if copy_res < 0 {
                panic!("cloning Dictionary failed");
            }
        }
    }
}

impl<'d> Drop for Dictionary<'d> {
    fn drop(&mut self) {
        if let Self::Owned(dict) = self {
            unsafe {
                av_dict_free(&mut dict.ptr);
            }
        }
    }
}

mod iter;
pub use self::iter::Iter;

#[macro_export]
macro_rules! dict {
    ( $($key:expr => $value:expr),* $(,)*) => ({
            let mut dict = ::ffmpeg::Dictionary::new();

            $(
                dict.set($key, $value);
            )*

            dict
        }
    );
}
