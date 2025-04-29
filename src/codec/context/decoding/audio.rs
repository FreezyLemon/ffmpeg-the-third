#[cfg(not(feature = "ffmpeg_5_0"))]
use crate::{
    ffi::*,
    frame,
    packet,
    Error
};
#[cfg(not(feature = "ffmpeg_5_0"))]
use libc::c_int;

#[cfg(not(feature = "ffmpeg_7_0"))]
use crate::ChannelLayoutMask;

use super::AudioDecoder;
use crate::{AsPtr, AsMutPtr};
use crate::util::format;
use crate::AudioService;

#[cfg(feature = "ffmpeg_5_1")]
use crate::ChannelLayout;

impl<S> AudioDecoder<S> {
    pub fn rate(&self) -> u32 {
        unsafe { (*self.as_ptr()).sample_rate as u32 }
    }

    #[cfg(not(feature = "ffmpeg_7_0"))]
    pub fn channels(&self) -> u16 {
        unsafe { (*self.as_ptr()).channels as u16 }
    }

    pub fn format(&self) -> format::Sample {
        unsafe { format::Sample::from((*self.as_ptr()).sample_fmt) }
    }

    pub fn request_format(&mut self, value: format::Sample) {
        unsafe {
            (*self.as_mut_ptr()).request_sample_fmt = value.into();
        }
    }

    #[cfg(not(feature = "ffmpeg_7_0"))]
    pub fn frames(&self) -> usize {
        unsafe { (*self.as_ptr()).frame_number as usize }
    }

    pub fn align(&self) -> usize {
        unsafe { (*self.as_ptr()).block_align as usize }
    }

    #[cfg(not(feature = "ffmpeg_7_0"))]
    pub fn channel_layout(&self) -> ChannelLayoutMask {
        unsafe { ChannelLayoutMask::from_bits_truncate((*self.as_ptr()).channel_layout) }
    }

    #[cfg(not(feature = "ffmpeg_7_0"))]
    pub fn set_channel_layout(&mut self, value: ChannelLayoutMask) {
        unsafe {
            (*self.as_mut_ptr()).channel_layout = value.bits();
        }
    }

    #[cfg(not(feature = "ffmpeg_7_0"))]
    pub fn request_channel_layout(&mut self, value: ChannelLayoutMask) {
        unsafe {
            (*self.as_mut_ptr()).request_channel_layout = value.bits();
        }
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

    pub fn audio_service(&mut self) -> AudioService {
        unsafe { AudioService::from((*self.as_mut_ptr()).audio_service_type) }
    }

    pub fn max_bit_rate(&self) -> usize {
        unsafe { (*self.as_ptr()).rc_max_rate as usize }
    }

    pub fn frame_size(&self) -> u32 {
        unsafe { (*self.as_ptr()).frame_size as u32 }
    }

    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn frame_start(&self) -> Option<usize> {
        unsafe {
            // Removed in ffmpeg >= 5.0 in favor of using encoder
            // private options.
            match (*self.as_ptr()).timecode_frame_start {
                -1 => None,
                n => Some(n as usize),
            }
        }
    }
}
