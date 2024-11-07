use super::{CodecType, Context, Decoding, State};
use crate::{chroma, color, format, FieldOrder, Rational};
use libc::c_int;

impl<C: CodecType, S: State> Context<Decoding, C, S> {
    // MUST be set
    pub fn set_time_base(&mut self, time_base: Rational) {
        unsafe {
            (*self.as_mut_ptr()).time_base = time_base.into();
        }
    }

    pub fn set_pkt_timebase(&mut self, pkt_timebase: Rational) {
        unsafe {
            (*self.as_mut_ptr()).pkt_timebase = pkt_timebase.into();
        }
    }

    // {0, 1} if unknown, otherwise signals framerate
    pub fn framerate(&self) -> Rational {
        unsafe { (*self.as_ptr()).framerate.into() }
    }

    // Number of frames delay IN ADDITION to a spec-conforming decoder
    // audio: number of samples the decoder needs to output before output is valid
    pub fn delay(&self) -> i32 {
        unsafe { (*self.as_ptr()).delay }
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

    pub fn sample_aspect_ratio(&self) -> Rational {
        unsafe { (*self.as_ptr()).sample_aspect_ratio.into() }
    }

    pub fn pix_fmt(&self) -> format::Pixel {
        unsafe { (*self.as_ptr()).pix_fmt.into() }
    }

    pub fn set_pix_fmt(&mut self, pix_fmt: format::Pixel) {
        unsafe { (*self.as_mut_ptr()).pix_fmt = pix_fmt.into() }
    }

    // TODO: sw_pix_fmt?

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

    pub fn set_color_range(&mut self, range: color::Range) {
        unsafe { (*self.as_mut_ptr()).color_range = range.into() }
    }

    pub fn chroma_sample_location(&self) -> chroma::Location {
        unsafe { (*self.as_ptr()).chroma_sample_location.into() }
    }

    pub fn set_field_order(&mut self, order: FieldOrder) {
        unsafe { (*self.as_mut_ptr()).field_order = order.into() }
    }

    pub fn reference_frames(&self) -> i32 {
        unsafe { (*self.as_ptr()).refs as i32 }
    }

    pub fn has_b_frames(&self) -> i32 {
        unsafe { (*self.as_ptr()).has_b_frames as i32 }
    }

    // TODO: slice_flags
    // TODO: draw_horiz_band
    // TODO: get_format
}
