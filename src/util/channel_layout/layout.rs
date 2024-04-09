use std::ffi::CString;

use crate::{ffi::*, Error};
use libc::{c_int, c_uint};

use super::{channel::Channel, mask::ChannelMask};

#[derive(Clone, PartialEq)]
#[repr(transparent)]
pub struct ChannelLayout(AVChannelLayout);

// TODO: Builder pattern for creating the underlying AVChannelOrder.
// Invariants are mostly based on the AVChannelOrder.

impl ChannelLayout {
    pub fn default_for_channels(channels: c_int) -> Self {
        let mut layout = AVChannelLayout::empty();
        unsafe {
            av_channel_layout_default(&mut layout as _, channels);
        }

        Self(layout)
    }

    pub fn from_mask(layout_mask: ChannelMask) -> Option<Self> {
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

    pub fn subset(&self, mask: ChannelMask) -> ChannelMask {
        ChannelMask::from_bits_truncate(unsafe {
            av_channel_layout_subset(&self.0 as _, mask.bits())
        })
    }

    pub fn check(&self) -> bool {
        unsafe { av_channel_layout_check(&self.0 as _) != 0 }
    }
}

impl From<AVChannelLayout> for ChannelLayout {
    fn from(value: AVChannelLayout) -> Self {
        Self(value)
    }
}

impl From<ChannelLayout> for AVChannelLayout {
    fn from(value: ChannelLayout) -> Self {
        value.0
    }
}
