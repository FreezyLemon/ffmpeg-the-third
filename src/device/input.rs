use std::ptr;

use crate::ffi::*;
use crate::format;

pub struct AudioIter(*const AVInputFormat);

impl Iterator for AudioIter {
    type Item = format::Input<'static>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            let ptr = av_input_audio_device_next(self.0);

            if !ptr.is_null() {
                self.0 = ptr;
            }

            format::Input::from_ptr(ptr)
        }
    }
}

pub fn audio() -> AudioIter {
    AudioIter(ptr::null())
}

pub struct VideoIter(*const AVInputFormat);

impl Iterator for VideoIter {
    type Item = format::Input<'static>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            let ptr = av_input_video_device_next(self.0);

            if !ptr.is_null() {
                self.0 = ptr;
            }

            format::Input::from_ptr(ptr)
        }
    }
}

pub fn video() -> VideoIter {
    VideoIter(ptr::null())
}
