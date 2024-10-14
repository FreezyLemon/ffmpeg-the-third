pub mod extensions;
pub mod input;
pub mod output;

use std::ffi::CStr;
use std::marker::PhantomData;
use std::str::from_utf8_unchecked;

use crate::ffi::*;
use crate::macros::impl_ref_wrapper;
use crate::media;

impl_ref_wrapper!(Info, AVDeviceInfo);

impl<'a> Info<'a> {
    pub fn name(&self) -> &str {
        unsafe { from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).device_name).to_bytes()) }
    }

    pub fn description(&self) -> &str {
        unsafe {
            from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).device_description).to_bytes())
        }
    }

    pub fn media_types(&self) -> MediaTypeIter {
        unsafe {
            MediaTypeIter::new(
                (*self.as_ptr()).media_types,
                (*self.as_ptr()).nb_media_types as usize,
            )
        }
    }
}

pub struct MediaTypeIter<'i> {
    curr: *const AVMediaType,
    remaining: usize,
    _marker: PhantomData<Info<'i>>,
}

impl<'i> MediaTypeIter<'i> {
    // TODO: Safety
    pub unsafe fn new(ptr: *const AVMediaType, remaining: usize) -> Self {
        Self {
            curr: ptr,
            remaining,
            _marker: PhantomData,
        }
    }
}

impl<'i> Iterator for MediaTypeIter<'i> {
    type Item = media::Type;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;

        unsafe {
            let media_type = (*self.curr).into();
            self.curr = self.curr.add(1);

            Some(media_type)
        }
    }
}

pub fn register_all() {
    unsafe {
        avdevice_register_all();
    }
}

pub fn version() -> u32 {
    unsafe { avdevice_version() }
}

pub fn configuration() -> &'static str {
    unsafe { from_utf8_unchecked(CStr::from_ptr(avdevice_configuration()).to_bytes()) }
}

pub fn license() -> &'static str {
    unsafe { from_utf8_unchecked(CStr::from_ptr(avdevice_license()).to_bytes()) }
}
