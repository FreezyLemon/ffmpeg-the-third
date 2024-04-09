use std::{mem::MaybeUninit, ptr::NonNull};

use ffmpeg_sys_the_third::*;

use libc::c_void;
use AVChannelOrder::*;
use ChannelOrder::*;

enum ChannelOrder {
    Unspecified,
    Native,
    Custom,
    Ambisonic,
}

impl From<AVChannelOrder> for ChannelOrder {
    fn from(value: AVChannelOrder) -> Self {
        match value {
            AV_CHANNEL_ORDER_NATIVE => Native,
            AV_CHANNEL_ORDER_UNSPEC => Unspecified,
            AV_CHANNEL_ORDER_CUSTOM => Custom,
            AV_CHANNEL_ORDER_AMBISONIC => Ambisonic,

            #[cfg(feature = "non-exhaustive-enums")]
            _ => unimplemented!(),
        }
    }
}

impl From<ChannelOrder> for AVChannelOrder {
    fn from(value: ChannelOrder) -> Self {
        match value {
            Unspecified => AV_CHANNEL_ORDER_UNSPEC,
            Native => AV_CHANNEL_ORDER_NATIVE,
            Custom => AV_CHANNEL_ORDER_CUSTOM,
            Ambisonic => AV_CHANNEL_ORDER_AMBISONIC,
        }
    }
}

struct ChannelCustom {
    ptr: NonNull<AVChannelCustom>,
}

enum ChannelLayoutInner {
    Mask(u64),
    Map(ChannelCustom),
}

struct ChannelLayout {
    inner: AVChannelLayout,
}

impl ChannelLayout {
    fn empty() -> Self {
        
    }

    fn default_for_channels(count: i32) -> Self {
        MaybeUninit::<AVChannelLayout>::uninit();
        unsafe {
            av_channel_layout_default(ch_layout, nb_channels)
        }
    }
    fn new() -> Self {
        unsafe {
            av_channel_layout_from_mask(channel_layout, mask)
        }
    }
}
