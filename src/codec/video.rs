use std::ops::Deref;

use super::codec::Codec;
use crate::ffi::*;
use crate::{format, Rational};

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Video {
    codec: Codec,
}

impl Video {
    pub fn new(codec: Codec) -> Video {
        Video { codec }
    }

    pub fn rates(&self) -> Option<RateIter> {
        if self.av_codec.supported_framerates.is_null() {
            None
        } else {
            Some(RateIter::new(self.av_codec.supported_framerates))
        }
    }

    pub fn formats(&self) -> Option<FormatIter> {
        if self.av_codec.pix_fmts.is_null() {
            None
        } else {
            Some(FormatIter::new(self.av_codec.pix_fmts))
        }
    }
}

impl Deref for Video {
    type Target = Codec;

    fn deref(&self) -> &Self::Target {
        &self.codec
    }
}

pub struct RateIter {
    ptr: *const AVRational,
}

impl RateIter {
    pub fn new(ptr: *const AVRational) -> Self {
        RateIter { ptr }
    }
}

impl Iterator for RateIter {
    type Item = Rational;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if (*self.ptr).num == 0 && (*self.ptr).den == 0 {
                return None;
            }

            let rate = (*self.ptr).into();
            self.ptr = self.ptr.offset(1);

            Some(rate)
        }
    }
}

pub struct FormatIter {
    ptr: *const AVPixelFormat,
}

impl FormatIter {
    pub fn new(ptr: *const AVPixelFormat) -> Self {
        FormatIter { ptr }
    }
}

impl Iterator for FormatIter {
    type Item = format::Pixel;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if *self.ptr == AVPixelFormat::AV_PIX_FMT_NONE {
                return None;
            }

            let format = (*self.ptr).into();
            self.ptr = self.ptr.offset(1);

            Some(format)
        }
    }
}
