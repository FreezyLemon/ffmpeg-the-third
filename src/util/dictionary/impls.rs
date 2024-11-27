use std::ffi::CString;

use super::flag::Flags;
use crate::{ffi::*, utils};

pub fn set(dict: &mut *mut AVDictionary, key: &str, value: &str, flags: Flags) {
    let key = CString::new(key).unwrap();
    let value = CString::new(value).unwrap();

    unsafe {
        if av_dict_set(dict, key.as_ptr(), value.as_ptr(), flags.bits()) < 0 {
            panic!("out of memory");
        }
    }
}

pub unsafe fn get<'d>(dict: *const AVDictionary, key: &str, flags: Flags) -> Option<&'d str> {
    let key = CString::new(key).unwrap();
    unsafe {
        let entry = av_dict_get(dict, key.as_ptr(), std::ptr::null_mut(), flags.bits());

        if entry.is_null() {
            None
        } else {
            Some(utils::str_from_c_ptr((*entry).value))
        }
    }
}
