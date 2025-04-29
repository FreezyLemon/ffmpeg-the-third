#[cfg(not(feature = "ffmpeg_5_0"))]
use libc::c_int;
#[cfg(not(feature = "ffmpeg_5_0"))]
use crate::{frame, packet};

#[cfg(not(feature = "ffmpeg_7_0"))]
use crate::ChannelLayoutMask;

use super::AudioEncoder;

use crate::{AsPtr, AsMutPtr};
use crate::util::format;

#[cfg(feature = "ffmpeg_5_1")]
use crate::ChannelLayout;

impl<S> AudioEncoder<S> {
    pub fn set_rate(&mut self, rate: i32) {
        unsafe {
            (*self.as_mut_ptr()).sample_rate = rate;
        }
    }

    pub fn rate(&self) -> u32 {
        unsafe { (*self.as_ptr()).sample_rate as u32 }
    }

    pub fn set_format(&mut self, value: format::Sample) {
        unsafe {
            (*self.as_mut_ptr()).sample_fmt = value.into();
        }
    }

    pub fn format(&self) -> format::Sample {
        unsafe { format::Sample::from((*self.as_ptr()).sample_fmt) }
    }

    #[cfg(not(feature = "ffmpeg_7_0"))]
    pub fn set_channel_layout(&mut self, value: ChannelLayoutMask) {
        unsafe {
            (*self.as_mut_ptr()).channel_layout = value.bits();
        }
    }

    #[cfg(not(feature = "ffmpeg_7_0"))]
    pub fn channel_layout(&self) -> ChannelLayoutMask {
        unsafe { ChannelLayoutMask::from_bits_truncate((*self.as_ptr()).channel_layout) }
    }

    #[cfg(not(feature = "ffmpeg_7_0"))]
    pub fn set_channels(&mut self, value: i32) {
        unsafe {
            (*self.as_mut_ptr()).channels = value;
        }
    }

    #[cfg(not(feature = "ffmpeg_7_0"))]
    pub fn channels(&self) -> u16 {
        unsafe { (*self.as_ptr()).channels as u16 }
    }

    #[cfg(feature = "ffmpeg_5_1")]
    pub fn ch_layout(&self) -> ChannelLayout {
        unsafe { ChannelLayout::from(&self.as_ptr().as_ref().unwrap().ch_layout) }
    }

    #[cfg(feature = "ffmpeg_5_1")]
    pub fn set_ch_layout(&mut self, value: ChannelLayout) {
        unsafe {
            self.as_mut_ptr().as_mut().unwrap().ch_layout = value.into_owned();
        }
    }
}
