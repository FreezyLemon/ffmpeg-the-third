pub mod extensions;
pub mod input;
pub mod output;

use std::ffi::CStr;
use std::str::from_utf8_unchecked;

use crate::ffi::*;

pub struct Info<'a> {
    info: &'a AVDeviceInfo,
}

impl<'a> Info<'a> {
    pub unsafe fn new(ptr: *const AVDeviceInfo) -> Option<Self> {
        unsafe { ptr.as_ref().map(|info| Self { info }) }
    }

    pub const fn as_ref(&self) -> &AVDeviceInfo {
        self.info
    }
}

impl<'a> Info<'a> {
    pub fn name(&self) -> &str {
        unsafe { from_utf8_unchecked(CStr::from_ptr(self.info.device_name).to_bytes()) }
    }

    pub fn description(&self) -> &str {
        unsafe { from_utf8_unchecked(CStr::from_ptr(self.info.device_description).to_bytes()) }
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
