use std::ffi::CStr;
use std::str::from_utf8_unchecked;

use crate::ffi::*;
use crate::macros::impl_ref_wrapper;

impl_ref_wrapper!(Input, AVInputFormat);

impl<'i> Input<'i> {
    pub fn name(&self) -> &str {
        unsafe { from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).name).to_bytes()) }
    }

    pub fn description(&self) -> &str {
        unsafe {
            let long_name = (*self.as_ptr()).long_name;

            if long_name.is_null() {
                ""
            } else {
                from_utf8_unchecked(CStr::from_ptr(long_name).to_bytes())
            }
        }
    }

    pub fn extensions(&self) -> Vec<&str> {
        unsafe {
            let ptr = (*self.as_ptr()).extensions;

            if ptr.is_null() {
                Vec::new()
            } else {
                from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes())
                    .split(',')
                    .collect()
            }
        }
    }

    pub fn mime_types(&self) -> Vec<&str> {
        unsafe {
            let ptr = (*self.as_ptr()).mime_type;

            if ptr.is_null() {
                Vec::new()
            } else {
                from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes())
                    .split(',')
                    .collect()
            }
        }
    }
}
