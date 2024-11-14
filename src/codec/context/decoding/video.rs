use libc::c_int;

use crate::decoder::{slice, Check, Conceal};
use crate::{Discard, FieldOrder};

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

    pub fn set_error_concealment(&mut self, concealment: Conceal) {
        unsafe {
            (*self.as_mut_ptr()).error_concealment = concealment.bits();
        }
    }

    pub fn set_err_recognition(&mut self, value: Check) {
        unsafe {
            (*self.as_mut_ptr()).err_recognition = value.bits();
        }
    }

    pub fn set_skip_loop_filter(&mut self, value: Discard) {
        unsafe {
            (*self.as_mut_ptr()).skip_loop_filter = value.into();
        }
    }

    pub fn set_skip_idct(&mut self, value: Discard) {
        unsafe {
            (*self.as_mut_ptr()).skip_idct = value.into();
        }
    }

    pub fn set_skip_frame(&mut self, value: Discard) {
        unsafe {
            (*self.as_mut_ptr()).skip_frame = value.into();
        }
    }
}
