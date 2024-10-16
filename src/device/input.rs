use std::ptr;

use crate::ffi::*;
use crate::format;

pub struct AudioIter(*mut AVInputFormat);

impl Iterator for AudioIter {
    type Item = format::Input;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            let ptr = av_input_audio_device_next(self.0) as *mut AVInputFormat;

            if ptr.is_null() && !self.0.is_null() {
                None
            } else {
                self.0 = ptr;

                Some(format::Input::wrap(ptr))
            }
        }
    }
}

pub fn audio() -> AudioIter {
    AudioIter(ptr::null_mut())
}

pub struct VideoIter(*mut AVInputFormat);

impl Iterator for VideoIter {
    type Item = format::Input;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            let ptr = av_input_video_device_next(self.0) as *mut AVInputFormat;

            if ptr.is_null() && !self.0.is_null() {
                None
            } else {
                self.0 = ptr;

                Some(format::Input::wrap(ptr))
            }
        }
    }
}

pub fn video() -> VideoIter {
    VideoIter(ptr::null_mut())
}
