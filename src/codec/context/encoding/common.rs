use crate::codec::codec::CodecType;
use crate::codec::context::{Context, Encoding, State};
use crate::{chroma, color, Rational};
use libc::c_int;

impl<C: CodecType, S: State> Context<Encoding, C, S> {
    // MUST be set
    pub fn set_time_base(&mut self, time_base: Rational) {
        unsafe {
            (*self.as_mut_ptr()).time_base = time_base.into();
        }
    }

    // Optional for CFR content
    pub fn set_framerate(&mut self, framerate: Rational) {
        unsafe {
            (*self.as_mut_ptr()).framerate = framerate.into();
        }
    }

    // optional? 0 if unknown
    // ratio=(width of pixel).div(height of pixel)
    pub fn set_sample_aspect_ratio(&mut self, ratio: Rational) {
        unsafe { (*self.as_mut_ptr()).sample_aspect_ratio = ratio.into() }
    }

    pub fn set_color_primaries(&mut self, primaries: color::Primaries) {
        unsafe { (*self.as_mut_ptr()).color_primaries = primaries.into() }
    }

    pub fn set_color_transfer(&mut self, transfer: color::TransferCharacteristic) {
        unsafe { (*self.as_mut_ptr()).color_trc = transfer.into() }
    }

    pub fn set_color_space(&mut self, space: color::Space) {
        unsafe { (*self.as_mut_ptr()).colorspace = space.into() }
    }

    pub fn set_chroma_sample_location(&mut self, location: chroma::Location) {
        unsafe { (*self.as_mut_ptr()).chroma_sample_location = location.into() }
    }

    pub fn set_reference_frames(&mut self, frames: i32) {
        unsafe { (*self.as_mut_ptr()).refs = frames as c_int }
    }
}
