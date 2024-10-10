use std::ops::Deref;

use super::codec::Codec;
use crate::codec::config::IterFromRef;

#[cfg(not(feature = "ffmpeg_7_0"))]
use crate::ChannelLayoutMask;

#[cfg(feature = "ffmpeg_7_1")]
use super::config::Supported;

pub use crate::codec::config::ChannelLayoutIter;
pub use crate::codec::config::SampleFormatIter as FormatIter;
pub use crate::codec::config::SampleRateIter as RateIter;

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Audio {
    codec: Codec,
}

impl Audio {
    pub unsafe fn new(codec: Codec) -> Audio {
        Audio { codec }
    }
}

impl Audio {
    /// Checks if the given sample rate is supported by this audio codec.
    #[cfg(feature = "ffmpeg_7_1")]
    pub fn supports_rate(&self, rate: libc::c_int) -> bool {
        self.supported_rates().supports(rate)
    }

    /// Returns a [`Supported`] representing the supported sample rates.
    #[cfg(feature = "ffmpeg_7_1")]
    pub fn supported_rates(&self) -> Supported<RateIter> {
        use super::config::supported_sample_rates;

        supported_sample_rates(self, None).expect("audio codec returns supported sample rates")
    }

    pub fn rates(&self) -> Option<RateIter> {
        unsafe {
            (*self.as_ptr())
                .supported_samplerates
                .as_ref()
                .map(|sr| RateIter::from_ref(sr))
        }
    }

    /// Checks if the given sample format is supported by this audio codec.
    #[cfg(feature = "ffmpeg_7_1")]
    pub fn supports_format(&self, format: crate::format::Sample) -> bool {
        self.supported_formats().supports(format)
    }

    /// Returns a [`Supported`] representing the supported sample formats.
    #[cfg(feature = "ffmpeg_7_1")]
    pub fn supported_formats(&self) -> Supported<FormatIter> {
        use super::config::supported_sample_formats;

        supported_sample_formats(self, None).expect("audio codec returns supported sample formats")
    }

    pub fn formats(&self) -> Option<FormatIter> {
        unsafe {
            (*self.as_ptr())
                .sample_fmts
                .as_ref()
                .map(|sf| FormatIter::from_ref(sf))
        }
    }

    #[cfg(not(feature = "ffmpeg_7_0"))]
    pub fn channel_layouts(&self) -> Option<ChannelLayoutMaskIter> {
        unsafe {
            if (*self.codec.as_ptr()).channel_layouts.is_null() {
                None
            } else {
                Some(ChannelLayoutMaskIter::new(
                    (*self.codec.as_ptr()).channel_layouts,
                ))
            }
        }
    }

    #[cfg(feature = "ffmpeg_5_1")]
    pub fn ch_layouts(&self) -> Option<ChannelLayoutIter> {
        unsafe {
            (*self.codec.as_ptr())
                .ch_layouts
                .as_ref()
                .map(|cl| ChannelLayoutIter::from_ref(cl))
        }
    }
}

impl Deref for Audio {
    type Target = Codec;

    fn deref(&self) -> &Self::Target {
        &self.codec
    }
}

#[cfg(not(feature = "ffmpeg_7_0"))]
pub struct ChannelLayoutMaskIter {
    ptr: *const u64,
}

#[cfg(not(feature = "ffmpeg_7_0"))]
impl ChannelLayoutMaskIter {
    pub fn new(ptr: *const u64) -> Self {
        ChannelLayoutMaskIter { ptr }
    }

    pub fn best(self, max: i32) -> ChannelLayoutMask {
        self.fold(ChannelLayoutMask::MONO, |acc, cur| {
            if cur.channels() > acc.channels() && cur.channels() <= max {
                cur
            } else {
                acc
            }
        })
    }
}

#[cfg(not(feature = "ffmpeg_7_0"))]
impl Iterator for ChannelLayoutMaskIter {
    type Item = ChannelLayoutMask;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if *self.ptr == 0 {
                return None;
            }

            let layout = ChannelLayoutMask::from_bits_truncate(*self.ptr);
            self.ptr = self.ptr.offset(1);

            Some(layout)
        }
    }
}

#[cfg(feature = "ffmpeg_5_1")]
impl<'a> ChannelLayoutIter<'a> {
    pub fn best(self, max: u32) -> crate::ChannelLayout<'a> {
        self.fold(crate::ChannelLayout::MONO, |acc, cur| {
            if cur.channels() > acc.channels() && cur.channels() <= max {
                cur
            } else {
                acc
            }
        })
    }
}
