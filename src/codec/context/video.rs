use libc::c_int;

use crate::ffi::AVRational;
use crate::{chroma, color, format, FieldOrder, Rational};

use super::{Action, Context, State, VideoType};

impl<A: Action, S: State> Context<A, VideoType, S> {
    pub fn width(&self) -> u32 {
        unsafe { (*self.as_ptr()).width as u32 }
    }

    pub fn height(&self) -> u32 {
        unsafe { (*self.as_ptr()).height as u32 }
    }

    pub fn has_b_frames(&self) -> bool {
        unsafe { (*self.as_ptr()).has_b_frames != 0 }
    }

    pub fn field_order(&self) -> FieldOrder {
        unsafe { (*self.as_ptr()).field_order.into() }
    }

    pub fn coded_width(&self) -> i32 {
        unsafe { (*self.as_ptr()).coded_width }
    }

    pub fn coded_height(&self) -> i32 {
        unsafe { (*self.as_ptr()).coded_height }
    }

    // Number of frames delay between encoder input and (spec-conforming) decoder output.
    // Unused for audio (see initial_padding)
    pub fn delay(&self) -> i32 {
        unsafe { (*self.as_ptr()).delay }
    }

    // Some(framerate) or None if unknown
    pub fn framerate(&self) -> Option<Rational> {
        let value = unsafe { (*self.as_ptr()).framerate };

        // {0, 1} if unknown
        if value == (AVRational { num: 0, den: 1 }) {
            None
        } else {
            Some(value.into())
        }
    }

    pub fn sample_aspect_ratio(&self) -> Rational {
        unsafe { (*self.as_ptr()).sample_aspect_ratio.into() }
    }

    pub fn pix_fmt(&self) -> format::Pixel {
        unsafe { (*self.as_ptr()).pix_fmt.into() }
    }

    pub fn color_primaries(&self) -> color::Primaries {
        unsafe { (*self.as_ptr()).color_primaries.into() }
    }

    pub fn color_transfer(&self) -> color::TransferCharacteristic {
        unsafe { (*self.as_ptr()).color_trc.into() }
    }

    pub fn color_space(&self) -> color::Space {
        unsafe { (*self.as_ptr()).colorspace.into() }
    }

    pub fn color_range(&self) -> color::Range {
        unsafe { (*self.as_ptr()).color_range.into() }
    }

    pub fn chroma_sample_location(&self) -> chroma::Location {
        unsafe { (*self.as_ptr()).chroma_sample_location.into() }
    }

    pub fn reference_frames(&self) -> i32 {
        unsafe { (*self.as_ptr()).refs as i32 }
    }

    // intra_matrix
    // inter_matrix

    pub fn intra_dc_precision(&self) -> u8 {
        unsafe { (*self.as_ptr()).intra_dc_precision as u8 }
    }

    // required for some decoders, optional otherwise
    pub fn set_width(&mut self, width: i32) {
        unsafe {
            (*self.as_mut_ptr()).width = width as c_int;
        }
    }

    // required for some decoders, optional otherwise
    pub fn set_height(&mut self, height: i32) {
        unsafe {
            (*self.as_mut_ptr()).height = height as c_int;
        }
    }

    pub fn set_pix_fmt(&mut self, pix_fmt: format::Pixel) {
        unsafe { (*self.as_mut_ptr()).pix_fmt = pix_fmt.into() }
    }

    // TODO: sw_pix_fmt?

    pub fn set_color_range(&mut self, range: color::Range) {
        unsafe { (*self.as_mut_ptr()).color_range = range.into() }
    }

    // TODO: slice_flags
    // TODO: draw_horiz_band
    // TODO: get_format
}
