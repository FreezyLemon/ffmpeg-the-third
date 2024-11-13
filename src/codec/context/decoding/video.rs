use libc::c_int;

use crate::decoder::slice;
use crate::FieldOrder;

use super::{State, VideoDecoder};

impl<S: State> VideoDecoder<S> {
    #[cfg(not(feature = "ffmpeg_7_0"))]
    pub fn set_slice_count(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).slice_count = value as c_int;
        }
    }

    pub fn set_slice_flags(&mut self, value: slice::Flags) {
        unsafe {
            (*self.as_mut_ptr()).slice_flags = value.bits();
        }
    }

    pub fn skip_top(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).skip_top = value as c_int;
        }
    }

    pub fn skip_bottom(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).skip_bottom = value as c_int;
        }
    }

    pub fn set_field_order(&mut self, value: FieldOrder) {
        unsafe {
            (*self.as_mut_ptr()).field_order = value.into();
        }
    }
}
