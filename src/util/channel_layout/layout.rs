use std::ffi::CString;

use crate::ffi::channel_layout::*;
use crate::ffi::*;
use crate::Error;
use libc::{c_int, c_uint};

use super::{channel::Channel, mask::ChannelLayout};

#[derive(Clone, PartialEq)]
#[repr(transparent)]
pub struct ChannelLayoutInfo(AVChannelLayout);

// TODO: Builder pattern for creating the underlying AVChannelLayout.
// Invariants are mostly based on the AVChannelOrder.

impl ChannelLayoutInfo {
    pub fn default_for_channels(channels: c_int) -> Self {
        let mut layout = AVChannelLayout::empty();
        unsafe {
            av_channel_layout_default(&mut layout as _, channels);
        }

        Self(layout)
    }

    pub fn from_mask(layout_mask: ChannelLayout) -> Option<Self> {
        let mut layout = AVChannelLayout::empty();
        let ret = unsafe { av_channel_layout_from_mask(&mut layout as _, layout_mask.bits()) };

        match ret {
            0 => Some(Self(layout)),
            r if r == AVERROR(EINVAL) => None,
            r => panic!("unexpected return value {r}"),
        }
    }

    pub fn from_string<S: Into<Vec<u8>>>(description: S) -> Option<Self> {
        let mut layout = AVChannelLayout::empty();
        let cstr = CString::new(description).expect("no nul byte in description");
        let ret = unsafe { av_channel_layout_from_string(&mut layout as _, cstr.as_ptr()) };

        match ret {
            0 => Some(Self(layout)),
            AVERROR_INVALIDDATA => None,
            r => panic!("unexpected return value {r}"),
        }
    }

    pub fn count(&self) -> c_int {
        self.0.nb_channels
    }

    pub fn describe(&self) -> Result<String, Error> {
        let mut buf = vec![0u8; 256];

        unsafe {
            let ret_val =
                av_channel_layout_describe(&self.0 as _, buf.as_mut_ptr() as _, buf.len());

            match usize::try_from(ret_val) {
                Ok(out_len) => {
                    buf.truncate(out_len);
                    Ok(String::from_utf8_unchecked(buf))
                }
                Err(_) => Err(Error::from(ret_val)),
            }
        }
    }

    pub fn channel_from_index(&self, idx: c_uint) -> Channel {
        Channel::from(unsafe { av_channel_layout_channel_from_index(&self.0 as _, idx) })
    }

    pub fn index_from_channel(&self, channel: Channel) -> c_int {
        unsafe { av_channel_layout_index_from_channel(&self.0 as _, AVChannel::from(channel)) }
    }

    pub fn index_from_string<S: Into<Vec<u8>>>(&self, name: S) -> Result<c_uint, Error> {
        let cstr = CString::new(name).expect("no nul byte in name");
        let ret = unsafe { av_channel_layout_index_from_string(&self.0 as _, cstr.as_ptr()) };

        match c_uint::try_from(ret) {
            Ok(idx) => Ok(idx),
            Err(_) => Err(Error::from(ret)),
        }
    }

    pub fn channel_from_string<S: Into<Vec<u8>>>(&self, name: S) -> Channel {
        let cstr = CString::new(name).expect("no nul byte in name");

        Channel::from(unsafe { av_channel_layout_channel_from_string(&self.0 as _, cstr.as_ptr()) })
    }

    pub fn subset(&self, mask: ChannelLayout) -> ChannelLayout {
        ChannelLayout::from_bits_truncate(unsafe {
            av_channel_layout_subset(&self.0 as _, mask.bits())
        })
    }

    pub fn check(&self) -> bool {
        unsafe { av_channel_layout_check(&self.0 as _) != 0 }
    }
}

impl From<AVChannelLayout> for ChannelLayoutInfo {
    fn from(value: AVChannelLayout) -> Self {
        Self(value)
    }
}

impl From<ChannelLayoutInfo> for AVChannelLayout {
    fn from(value: ChannelLayoutInfo) -> Self {
        value.0
    }
}

// Constants
impl ChannelLayoutInfo {
    pub const MONO: Self = Self(AV_CHANNEL_LAYOUT_MONO);
    pub const STEREO: Self = Self(AV_CHANNEL_LAYOUT_STEREO);
    pub const _2POINT1: Self = Self(AV_CHANNEL_LAYOUT_2POINT1);
    pub const _2_1: Self = Self(AV_CHANNEL_LAYOUT_2_1);
    pub const SURROUND: Self = Self(AV_CHANNEL_LAYOUT_SURROUND);
    pub const _3POINT1: Self = Self(AV_CHANNEL_LAYOUT_3POINT1);
    pub const _4POINT0: Self = Self(AV_CHANNEL_LAYOUT_4POINT0);
    pub const _4POINT1: Self = Self(AV_CHANNEL_LAYOUT_4POINT1);
    pub const _2_2: Self = Self(AV_CHANNEL_LAYOUT_2_2);
    pub const QUAD: Self = Self(AV_CHANNEL_LAYOUT_QUAD);
    pub const _5POINT0: Self = Self(AV_CHANNEL_LAYOUT_5POINT0);
    pub const _5POINT1: Self = Self(AV_CHANNEL_LAYOUT_5POINT1);
    pub const _5POINT0_BACK: Self = Self(AV_CHANNEL_LAYOUT_5POINT0_BACK);
    pub const _5POINT1_BACK: Self = Self(AV_CHANNEL_LAYOUT_5POINT1_BACK);
    pub const _6POINT0: Self = Self(AV_CHANNEL_LAYOUT_6POINT0);
    pub const _6POINT0_FRONT: Self = Self(AV_CHANNEL_LAYOUT_6POINT0_FRONT);
    pub const _3POINT1POINT2: Self = Self(AV_CHANNEL_LAYOUT_3POINT1POINT2);
    pub const HEXAGONAL: Self = Self(AV_CHANNEL_LAYOUT_HEXAGONAL);
    pub const _6POINT1: Self = Self(AV_CHANNEL_LAYOUT_6POINT1);
    pub const _6POINT1_BACK: Self = Self(AV_CHANNEL_LAYOUT_6POINT1_BACK);
    pub const _6POINT1_FRONT: Self = Self(AV_CHANNEL_LAYOUT_6POINT1_FRONT);
    pub const _7POINT0: Self = Self(AV_CHANNEL_LAYOUT_7POINT0);
    pub const _7POINT0_FRONT: Self = Self(AV_CHANNEL_LAYOUT_7POINT0_FRONT);
    pub const _7POINT1: Self = Self(AV_CHANNEL_LAYOUT_7POINT1);
    pub const _7POINT1_WIDE: Self = Self(AV_CHANNEL_LAYOUT_7POINT1_WIDE);
    pub const _7POINT1_WIDE_BACK: Self = Self(AV_CHANNEL_LAYOUT_7POINT1_WIDE_BACK);
    pub const _5POINT1POINT2_BACK: Self = Self(AV_CHANNEL_LAYOUT_5POINT1POINT2_BACK);
    pub const OCTAGONAL: Self = Self(AV_CHANNEL_LAYOUT_OCTAGONAL);
    pub const CUBE: Self = Self(AV_CHANNEL_LAYOUT_CUBE);
    pub const _5POINT1POINT4_BACK: Self = Self(AV_CHANNEL_LAYOUT_5POINT1POINT4_BACK);
    pub const _7POINT1POINT2: Self = Self(AV_CHANNEL_LAYOUT_7POINT1POINT2);
    pub const _7POINT1POINT4_BACK: Self = Self(AV_CHANNEL_LAYOUT_7POINT1POINT4_BACK);
    pub const HEXADECAGONAL: Self = Self(AV_CHANNEL_LAYOUT_HEXADECAGONAL);
    pub const STEREO_DOWNMIX: Self = Self(AV_CHANNEL_LAYOUT_STEREO_DOWNMIX);
    pub const _22POINT2: Self = Self(AV_CHANNEL_LAYOUT_22POINT2);
}
