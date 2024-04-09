use std::ffi::{CStr, CString};

use crate::{ffi::*, Error};
use libc::{c_char, c_int, c_uint};

use super::{channel::Channel, mask::ChannelMask};

#[derive(Clone)]
#[repr(transparent)]
pub struct ChannelLayoutInfo(AVChannelLayout);

impl ChannelLayoutInfo {
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
