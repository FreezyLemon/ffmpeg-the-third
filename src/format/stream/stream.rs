use super::Disposition;
use crate::codec;
use crate::ffi::*;
use crate::format::context::common::Context;
use crate::packet::side_data::SideDataIter;
use crate::{DictionaryRef, Discard, Rational};

#[derive(Debug)]
pub struct Stream<'a> {
    context: &'a Context,
    index: usize,
}

impl<'a> Stream<'a> {
    pub unsafe fn wrap(context: &Context, index: usize) -> Stream {
        Stream { context, index }
    }

    pub unsafe fn as_ptr(&self) -> *const AVStream {
        *(*self.context.as_ptr()).streams.add(self.index)
    }
}

impl<'a> Stream<'a> {
    pub fn id(&self) -> i32 {
        unsafe { (*self.as_ptr()).id }
    }

    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn codec(&self) -> codec::Context {
        unsafe { codec::Context::wrap((*self.as_ptr()).codec, Some(self.context.destructor())) }
    }

    pub fn parameters(&self) -> codec::Parameters {
        unsafe {
            codec::Parameters::wrap((*self.as_ptr()).codecpar, Some(self.context.destructor()))
        }
    }

    pub fn index(&self) -> usize {
        unsafe { (*self.as_ptr()).index as usize }
    }

    pub fn time_base(&self) -> Rational {
        unsafe { Rational::from((*self.as_ptr()).time_base) }
    }

    pub fn start_time(&self) -> i64 {
        unsafe { (*self.as_ptr()).start_time }
    }

    pub fn duration(&self) -> i64 {
        unsafe { (*self.as_ptr()).duration }
    }

    pub fn frames(&self) -> i64 {
        unsafe { (*self.as_ptr()).nb_frames }
    }

    pub fn disposition(&self) -> Disposition {
        unsafe { Disposition::from_bits_truncate((*self.as_ptr()).disposition) }
    }

    pub fn discard(&self) -> Discard {
        unsafe { Discard::from((*self.as_ptr()).discard) }
    }

    pub fn side_data(&self) -> SideDataIter {
        unsafe { SideDataIter::new((*self.as_ptr()).side_data, (*self.as_ptr()).nb_side_data) }
    }

    pub fn rate(&self) -> Rational {
        unsafe { Rational::from((*self.as_ptr()).r_frame_rate) }
    }

    pub fn avg_frame_rate(&self) -> Rational {
        unsafe { Rational::from((*self.as_ptr()).avg_frame_rate) }
    }

    pub fn metadata(&self) -> DictionaryRef {
        unsafe { DictionaryRef::wrap((*self.as_ptr()).metadata) }
    }
}

impl<'a> PartialEq for Stream<'a> {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.as_ptr() == other.as_ptr() }
    }
}

impl<'a> Eq for Stream<'a> {}
