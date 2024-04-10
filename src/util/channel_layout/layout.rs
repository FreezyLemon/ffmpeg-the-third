use std::borrow::Borrow;
use std::borrow::Cow;
use std::ffi::CString;

use crate::ffi::*;
use crate::Error;
use libc::{c_int, c_uint};

use super::{channel::Channel, mask::ChannelLayout};

#[derive(Debug, Clone, PartialEq)]
#[repr(transparent)]
pub struct ChannelLayoutInfo<'a>(Cow<'a, AVChannelLayout>);

// TODO: Builder pattern for creating the underlying AVChannelLayout.
// Invariants are mostly based on the AVChannelOrder.

impl<'a> ChannelLayoutInfo<'a> {
    pub fn default_for_channels(channels: c_int) -> Self {
        let mut layout = AVChannelLayout::empty();
        unsafe {
            av_channel_layout_default(&mut layout as _, channels);
        }

        Self(Cow::Owned(layout))
    }

    pub fn from_mask(layout_mask: ChannelLayout) -> Option<Self> {
        let mut layout = AVChannelLayout::empty();
        let ret = unsafe { av_channel_layout_from_mask(&mut layout as _, layout_mask.bits()) };

        match ret {
            0 => Some(Self(Cow::Owned(layout))),
            r if r == AVERROR(EINVAL) => None,
            r => panic!("unexpected return value {r}"),
        }
    }

    pub fn from_string<S: Into<Vec<u8>>>(description: S) -> Option<Self> {
        let mut layout = AVChannelLayout::empty();
        let cstr = CString::new(description).expect("no nul byte in description");
        let ret = unsafe { av_channel_layout_from_string(&mut layout as _, cstr.as_ptr()) };

        match ret {
            0 => Some(Self(Cow::Owned(layout))),
            AVERROR_INVALIDDATA => None,
            r => panic!("unexpected return value {r}"),
        }
    }

    /// Extracts the owned `AVChannelLayout`.
    ///
    /// Clones it if not already owned.
    pub fn into_owned(self) -> AVChannelLayout {
        self.0.into_owned()
    }

    pub fn count(&self) -> c_int {
        self.0.nb_channels
    }

    /// Exposes a pointer the contained `AVChannelLayout` for FFI purposes.
    pub fn as_ptr(&self) -> *const AVChannelLayout {
        self.0.as_ref() as _
    }

    pub fn describe(&self) -> Result<String, Error> {
        let mut buf = vec![0u8; 256];

        unsafe {
            let ret_val =
                av_channel_layout_describe(self.as_ptr(), buf.as_mut_ptr() as _, buf.len());

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
        Channel::from(unsafe { av_channel_layout_channel_from_index(self.as_ptr(), idx) })
    }

    pub fn index_from_channel(&self, channel: Channel) -> c_int {
        unsafe { av_channel_layout_index_from_channel(self.as_ptr(), AVChannel::from(channel)) }
    }

    pub fn index_from_string<S: Into<Vec<u8>>>(&self, name: S) -> Result<c_uint, Error> {
        let cstr = CString::new(name).expect("no nul byte in name");
        let ret = unsafe { av_channel_layout_index_from_string(self.as_ptr(), cstr.as_ptr()) };

        match c_uint::try_from(ret) {
            Ok(idx) => Ok(idx),
            Err(_) => Err(Error::from(ret)),
        }
    }

    pub fn channel_from_string<S: Into<Vec<u8>>>(&self, name: S) -> Channel {
        let cstr = CString::new(name).expect("no nul byte in name");

        Channel::from(unsafe {
            av_channel_layout_channel_from_string(self.as_ptr(), cstr.as_ptr())
        })
    }

    pub fn subset(&self, mask: ChannelLayout) -> ChannelLayout {
        ChannelLayout::from_bits_truncate(unsafe {
            av_channel_layout_subset(self.as_ptr(), mask.bits())
        })
    }

    pub fn check(&self) -> bool {
        unsafe { av_channel_layout_check(self.as_ptr()) != 0 }
    }
}

impl<'a> From<AVChannelLayout> for ChannelLayoutInfo<'a> {
    fn from(value: AVChannelLayout) -> Self {
        Self(Cow::Owned(value))
    }
}

impl<'a> From<&'a AVChannelLayout> for ChannelLayoutInfo<'a> {
    fn from(value: &'a AVChannelLayout) -> Self {
        Self(Cow::Borrowed(value))
    }
}

impl<'a> Borrow<AVChannelLayout> for ChannelLayoutInfo<'a> {
    fn borrow(&self) -> &AVChannelLayout {
        &self.0
    }
}

use crate::ffi::channel_layout::*;

// Constants
impl<'a> ChannelLayoutInfo<'a> {
    pub const MONO: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_MONO));
    pub const STEREO: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_STEREO));
    pub const _2POINT1: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_2POINT1));
    pub const _2_1: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_2_1));
    pub const SURROUND: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_SURROUND));
    pub const _3POINT1: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_3POINT1));
    pub const _4POINT0: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_4POINT0));
    pub const _4POINT1: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_4POINT1));
    pub const _2_2: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_2_2));
    pub const QUAD: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_QUAD));
    pub const _5POINT0: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_5POINT0));
    pub const _5POINT1: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_5POINT1));
    pub const _5POINT0_BACK: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_5POINT0_BACK));
    pub const _5POINT1_BACK: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_5POINT1_BACK));
    pub const _6POINT0: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_6POINT0));
    pub const _6POINT0_FRONT: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_6POINT0_FRONT));
    pub const _3POINT1POINT2: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_3POINT1POINT2));
    pub const HEXAGONAL: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_HEXAGONAL));
    pub const _6POINT1: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_6POINT1));
    pub const _6POINT1_BACK: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_6POINT1_BACK));
    pub const _6POINT1_FRONT: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_6POINT1_FRONT));
    pub const _7POINT0: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_7POINT0));
    pub const _7POINT0_FRONT: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_7POINT0_FRONT));
    pub const _7POINT1: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_7POINT1));
    pub const _7POINT1_WIDE: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_7POINT1_WIDE));
    pub const _7POINT1_WIDE_BACK: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_7POINT1_WIDE_BACK));
    pub const _5POINT1POINT2_BACK: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_5POINT1POINT2_BACK));
    pub const OCTAGONAL: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_OCTAGONAL));
    pub const CUBE: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_CUBE));
    pub const _5POINT1POINT4_BACK: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_5POINT1POINT4_BACK));
    pub const _7POINT1POINT2: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_7POINT1POINT2));
    pub const _7POINT1POINT4_BACK: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_7POINT1POINT4_BACK));
    pub const HEXADECAGONAL: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_HEXADECAGONAL));
    pub const STEREO_DOWNMIX: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_STEREO_DOWNMIX));
    pub const _22POINT2: ChannelLayoutInfo<'static> =
        ChannelLayoutInfo(Cow::Owned(AV_CHANNEL_LAYOUT_22POINT2));
}
