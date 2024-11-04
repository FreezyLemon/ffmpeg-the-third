use super::{Context, Encoding};
use crate::{chroma, color, format, FieldOrder, Rational};
use libc::c_int;

impl<S> Context<Encoding, S> {
    // Optional for CFR content
    pub fn set_framerate(&mut self, framerate: Rational) {
        unsafe {
            (*self.as_mut_ptr()).framerate = framerate.into();
        }
    }

    // Number of frames delay between encoder input and (spec-conforming) decoder output.
    // Unused for audio (see initial_padding)
    pub fn delay(&self) -> i32 {
        unsafe { (*self.as_ptr()).delay }
    }

    pub fn width(&self) -> i32 {
        unsafe { (*self.as_ptr()).width as i32 }
    }

    // required
    pub fn set_width(&mut self, width: i32) {
        unsafe { (*self.as_mut_ptr()).width = width as c_int }
    }

    pub fn height(&self) -> i32 {
        unsafe { (*self.as_ptr()).height as i32 }
    }

    // required
    pub fn set_height(&mut self, height: i32) {
        unsafe { (*self.as_mut_ptr()).height = height as c_int }
    }

    pub fn coded_width(&self) -> i32 {
        unsafe { (*self.as_ptr()).coded_width }
    }

    pub fn coded_height(&self) -> i32 {
        unsafe { (*self.as_ptr()).coded_height }
    }

    // optional? 0 if unknown
    // ratio=(width of pixel).div(height of pixel)
    pub fn set_sample_aspect_ratio(&mut self, ratio: Rational) {
        unsafe { (*self.as_mut_ptr()).sample_aspect_ratio = ratio.into() }
    }

    pub fn set_pix_fmt(&mut self, pix_fmt: format::Pixel) {
        unsafe { (*self.as_mut_ptr()).pix_fmt = pix_fmt.into() }
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

    // set = override default
    pub fn set_color_range(&mut self, range: color::Range) {
        unsafe { (*self.as_mut_ptr()).color_range = range.into() }
    }

    pub fn set_chroma_sample_location(&mut self, location: chroma::Location) {
        unsafe { (*self.as_mut_ptr()).chroma_sample_location = location.into() }
    }

    pub fn field_order(&self) -> FieldOrder {
        unsafe { (*self.as_ptr()).field_order.into() }
    }

    pub fn set_reference_frames(&mut self, frames: i32) {
        unsafe { (*self.as_mut_ptr()).refs = frames as c_int }
    }

    pub fn has_b_frames(&self) -> i32 {
        unsafe { (*self.as_ptr()).has_b_frames as i32 }
    }
}
