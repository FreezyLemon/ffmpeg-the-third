use std::ffi::{CStr, CString};
use std::marker::PhantomData;

use libc::c_int;

use crate::{ffi::*, Error};

use super::DictionaryEntry;

// Important note:
// The header libavutil/dict.h defines `AVDictionary` to be a zero-sized struct, while
// the implementation in libavutil/dict.c privately has a different struct definition.
// This means that the auto-generated `AVDictionary` is a ZST which is wrong, but exactly
// what's intended by FFmpeg (you're supposed to always use pointers for AVDictionary).
// I am not sure if dereferencing a pointer to this ZST is UB if the pointer is otherwise
// valid (but for a different struct). This implementation should make sure that this never
// causes UB.
//
// This does not apply for `AVDictionaryEntry` which is perfectly fine to dereference etc.

#[derive(Debug)]
pub enum Dictionary<'dict> {
    Owned(DictionaryOwned),
    Borrowed(DictionaryBorrowed<'dict>),
    // BorrowedMut(DictionaryBorrowedMut<'dict>),
}

// Private structs
//
// SAFETY notes for implementers:
// - `ptr` must be either null (to indicate an empty dictionary) or a pointer returned/set
//   by the public FFmpeg API, e.g. `av_dict_set` or `av_dict_copy`.
// - Make sure to never dereference `ptr` or create a reference from it (see note above).
// - Make sure that Rust aliasing rules are enforced (PhantomData should help with this).
//   For example:
//   - DictionaryOwned MUST ensure the dictionary that is pointed to is really owned by us.
//   - DictionaryBorrowed MUST ensure the dictionary that is pointed to is never mutated
//     for the lifetime of the DictionaryBorrowed.
//   - DictionaryBorrowed MUST NOT mutate the dictionary in any way.
//   - DictionaryOwned and DictionaryBorrowedMut MUST NOT allow holding more than one mutable
//     reference to a dictionary at a time.

#[derive(Debug)]
struct DictionaryOwned {
    ptr: *mut AVDictionary,
    _marker: PhantomData<Option<AVDictionary>>,
}

#[derive(Debug)]
struct DictionaryBorrowed<'dict> {
    ptr: *const AVDictionary,
    _marker: PhantomData<Option<&'dict AVDictionary>>,
}

// #[derive(Debug)]
// struct DictionaryBorrowedMut<'dict> {
//     ptr: *mut AVDictionary,
//     _marker: PhantomData<Option<&'dict mut AVDictionary>>,
// }

impl<'dict> Dictionary<'dict> {
    pub fn empty() -> Self {
        // SAFETY: Calling owned with a null pointer is valid
        unsafe { Self::owned(std::ptr::null_mut()) }
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

    pub fn to_mut(&mut self) -> *mut AVDictionary {
        if let Self::Borrowed(_) = self {
            let clone = self.clone();
            *self = clone;
        }

        match self {
            Dictionary::Owned(d) => d.ptr,
            // Dictionary::BorrowedMut(d) => d.ptr,
            Dictionary::Borrowed(_) => unreachable!(),
        }
    }

    pub fn borrowed<'borrow>(&self) -> Dictionary<'borrow>
    where
        'dict: 'borrow,
    {
        Dictionary::Borrowed(DictionaryBorrowed {
            ptr: self.as_ptr(),
            _marker: PhantomData,
        })
    }

    // pub fn borrowed_mut<'borrow>(&mut self) -> Option<Dictionary<'borrow>>
    // where
    //     'dict: 'borrow,
    // {
    //     if let Self::Owned(d) = self {
    //         Some(Dictionary::BorrowedMut(DictionaryBorrowedMut {
    //             ptr: d.ptr,
    //             _marker: PhantomData,
    //         }))
    //     } else {
    //         None
    //     }
    // }

    pub fn as_ptr(&self) -> *const AVDictionary {
        match self {
            Self::Owned(d) => d.ptr,
            Self::Borrowed(d) => d.ptr,
            // Self::BorrowedMut(d) => d.ptr,
        }
    }

    fn get<'entry>(
        &self,
        key: &CStr,
        prev: Option<DictionaryEntry>,
        flags: c_int,
    ) -> Result<DictionaryEntry<'entry>, DictGetError>
    where
        'dict: 'entry,
    {
        let prev: *const AVDictionaryEntry = match prev {
            Some(entry) => entry.av_entry,
            None => std::ptr::null(),
        };

        unsafe {
            // SAFETY: `key.as_ptr()` will return a valid non-null pointer.
            // SAFETY: `prev` is guaranteed to be null or made from a reference (= valid) (see above).
            let av_entry_ptr = av_dict_get(self.as_ptr(), key.as_ptr(), prev, flags);

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
    pub fn get_single<K: Into<Vec<u8>>>(&self, key: K) -> Result<DictionaryEntry, DictGetError> {
        let key = CString::new(key).map_err(|_| DictGetError::InvalidKey)?;

        self.get(&key, None, 0)
    }

    pub fn get_many<K: Into<Vec<u8>>>(
        &self,
        key: K,
    ) -> Result<DictionaryGetManyIter, DictGetError> {
        let key = CString::new(key).map_err(|_| DictGetError::InvalidKey)?;

        Ok(DictionaryGetManyIter::new(self.borrowed(), key, 0))
    }

    fn set(&mut self, key: &CStr, value: Option<&CStr>, flags: c_int) -> Result<(), Error> {
        let value = match value {
            Some(cstr) => cstr.as_ptr(),
            None => std::ptr::null(),
        };

        let ret = unsafe {
            // TODO: SAFETY comment
            av_dict_set(&mut self.to_mut(), key.as_ptr(), value, flags)
        };

        if ret >= 0 {
            Ok(())
        } else {
            Err(Error::from(ret))
        }
    }

    pub fn set_single<K: Into<Vec<u8>>, V: Into<Vec<u8>>>(
        &mut self,
        key: K,
        value: V,
    ) -> Result<(), DictSetError> {
        let key = CString::new(key).map_err(|_| DictSetError::InvalidKey)?;
        let value = CString::new(value).map_err(|_| DictSetError::InvalidValue)?;

        self.set(&key, Some(&value), 0)
            .map_err(|err| DictSetError::Other(err))
    }

    pub fn remove<K: Into<Vec<u8>>>(&mut self, key: K) -> Result<(), DictSetError> {
        let key = CString::new(key).map_err(|_| DictSetError::InvalidKey)?;

        self.set(&key, None, 0)
            .map_err(|err| DictSetError::Other(err))
    }
}

pub enum DictGetError {
    NotFound,
    InvalidKey,
}

pub enum DictSetError {
    InvalidKey,
    InvalidValue,
    Other(Error),
}

impl<'d> Clone for Dictionary<'d> {
    fn clone(&self) -> Self {
        let mut ptr = std::ptr::null_mut();

        unsafe {
            // AV_DICT_MULTIKEY so that keys that exist multiple times will be kept.
            // SAFETY: dst is always non-null and valid because it's made from a reference,
            //         (*dst) being null is OK and intended
            // SAFETY: src is either null or
            let copy_res = av_dict_copy(&mut ptr, self.as_ptr(), AV_DICT_MULTIKEY);

            // Create owned dictionary before checking the return value
            // so that memory is properly freed if panicking
            let res = Self::owned(ptr);
            if copy_res < 0 {
                panic!("av_dict_copy failed");
            }

            res
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

pub struct DictionaryGetManyIter<'d: 'e, 'e> {
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

    fn next(&mut self) -> Option<Self::Item> {
        match self.dict.get(&self.key, self.prev, self.flags) {
            Ok(entry) => {
                self.prev = Some(entry);
                Some(entry)
            }
            Err(_) => None,
        }
    }
}
