use libc::c_int;

use crate::*;
use crate::avutil::channel_masks::*;
use crate::{AVChannelLayout, AVChannelOrder};

use std::alloc::{handle_alloc_error, Layout};
use std::mem::{align_of, size_of};
use std::ptr::null_mut;

impl AVChannelLayout {
    #[inline]
    pub fn empty() -> Self {
        Self {
            order: AVChannelOrder::AV_CHANNEL_ORDER_UNSPEC,
            nb_channels: 0,
            u: AVChannelLayout__bindgen_ty_1 { mask: 0 },
            opaque: null_mut(),
        }
    }
}

impl Clone for AVChannelLayout {
    fn clone(&self) -> Self {
        let mut cloned = Self::empty();
        cloned.clone_from(self);

        cloned
    }

    fn clone_from(&mut self, source: &Self) {
        #[cold]
        fn clone_failed(channels: c_int) -> ! {
            let alloc_size = channels as usize * size_of::<AVChannelCustom>();
            let layout =
                Layout::from_size_align(alloc_size, align_of::<AVChannelCustom>())
                    .unwrap();
            handle_alloc_error(layout)
        }

        let ret = unsafe {
            av_channel_layout_copy(self as _, source as _)
        };

        if ret < 0 {
            clone_failed(self.nb_channels);
        }
    }
}

impl Drop for AVChannelLayout {
    fn drop(&mut self) {
        unsafe {
            av_channel_layout_uninit(self as _)
        }
    }
}

impl PartialEq for AVChannelLayout {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            av_channel_layout_compare(self as _, other as _) == 0
        }
    }
}


// Audio channel layouts as AVChannelLayout
pub const fn AV_CHANNEL_LAYOUT_MASK(nb_channels: c_int, channel_mask: u64) -> AVChannelLayout {
    AVChannelLayout {
        order: AVChannelOrder::AV_CHANNEL_ORDER_NATIVE,
        nb_channels,
        u: crate::AVChannelLayout__bindgen_ty_1 { mask: channel_mask },
        opaque: std::ptr::null_mut(),
    }
}

pub const AV_CHANNEL_LAYOUT_MONO: AVChannelLayout = AV_CHANNEL_LAYOUT_MASK(1, AV_CH_LAYOUT_MONO);
pub const AV_CHANNEL_LAYOUT_STEREO: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(2, AV_CH_LAYOUT_STEREO);
pub const AV_CHANNEL_LAYOUT_2POINT1: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(3, AV_CH_LAYOUT_2POINT1);
pub const AV_CHANNEL_LAYOUT_2_1: AVChannelLayout = AV_CHANNEL_LAYOUT_MASK(3, AV_CH_LAYOUT_2_1);
pub const AV_CHANNEL_LAYOUT_SURROUND: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(3, AV_CH_LAYOUT_SURROUND);
pub const AV_CHANNEL_LAYOUT_3POINT1: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(4, AV_CH_LAYOUT_3POINT1);
pub const AV_CHANNEL_LAYOUT_4POINT0: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(4, AV_CH_LAYOUT_4POINT0);
pub const AV_CHANNEL_LAYOUT_4POINT1: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(5, AV_CH_LAYOUT_4POINT1);
pub const AV_CHANNEL_LAYOUT_2_2: AVChannelLayout = AV_CHANNEL_LAYOUT_MASK(4, AV_CH_LAYOUT_2_2);
pub const AV_CHANNEL_LAYOUT_QUAD: AVChannelLayout = AV_CHANNEL_LAYOUT_MASK(4, AV_CH_LAYOUT_QUAD);
pub const AV_CHANNEL_LAYOUT_5POINT0: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(5, AV_CH_LAYOUT_5POINT0);
pub const AV_CHANNEL_LAYOUT_5POINT1: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(6, AV_CH_LAYOUT_5POINT1);
pub const AV_CHANNEL_LAYOUT_5POINT0_BACK: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(5, AV_CH_LAYOUT_5POINT0_BACK);
pub const AV_CHANNEL_LAYOUT_5POINT1_BACK: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(6, AV_CH_LAYOUT_5POINT1_BACK);
pub const AV_CHANNEL_LAYOUT_6POINT0: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(6, AV_CH_LAYOUT_6POINT0);
pub const AV_CHANNEL_LAYOUT_6POINT0_FRONT: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(6, AV_CH_LAYOUT_6POINT0_FRONT);
pub const AV_CHANNEL_LAYOUT_3POINT1POINT2: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(6, AV_CH_LAYOUT_3POINT1POINT2);
pub const AV_CHANNEL_LAYOUT_HEXAGONAL: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(6, AV_CH_LAYOUT_HEXAGONAL);
pub const AV_CHANNEL_LAYOUT_6POINT1: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(7, AV_CH_LAYOUT_6POINT1);
pub const AV_CHANNEL_LAYOUT_6POINT1_BACK: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(7, AV_CH_LAYOUT_6POINT1_BACK);
pub const AV_CHANNEL_LAYOUT_6POINT1_FRONT: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(7, AV_CH_LAYOUT_6POINT1_FRONT);
pub const AV_CHANNEL_LAYOUT_7POINT0: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(7, AV_CH_LAYOUT_7POINT0);
pub const AV_CHANNEL_LAYOUT_7POINT0_FRONT: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(7, AV_CH_LAYOUT_7POINT0_FRONT);
pub const AV_CHANNEL_LAYOUT_7POINT1: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(8, AV_CH_LAYOUT_7POINT1);
pub const AV_CHANNEL_LAYOUT_7POINT1_WIDE: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(8, AV_CH_LAYOUT_7POINT1_WIDE);
pub const AV_CHANNEL_LAYOUT_7POINT1_WIDE_BACK: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(8, AV_CH_LAYOUT_7POINT1_WIDE_BACK);
pub const AV_CHANNEL_LAYOUT_5POINT1POINT2_BACK: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(8, AV_CH_LAYOUT_5POINT1POINT2_BACK);
pub const AV_CHANNEL_LAYOUT_OCTAGONAL: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(8, AV_CH_LAYOUT_OCTAGONAL);
pub const AV_CHANNEL_LAYOUT_CUBE: AVChannelLayout = AV_CHANNEL_LAYOUT_MASK(8, AV_CH_LAYOUT_CUBE);
pub const AV_CHANNEL_LAYOUT_5POINT1POINT4_BACK: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(10, AV_CH_LAYOUT_5POINT1POINT4_BACK);
pub const AV_CHANNEL_LAYOUT_7POINT1POINT2: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(10, AV_CH_LAYOUT_7POINT1POINT2);
pub const AV_CHANNEL_LAYOUT_7POINT1POINT4_BACK: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(12, AV_CH_LAYOUT_7POINT1POINT4_BACK);
pub const AV_CHANNEL_LAYOUT_HEXADECAGONAL: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(16, AV_CH_LAYOUT_HEXADECAGONAL);
pub const AV_CHANNEL_LAYOUT_STEREO_DOWNMIX: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(2, AV_CH_LAYOUT_STEREO_DOWNMIX);
pub const AV_CHANNEL_LAYOUT_22POINT2: AVChannelLayout =
    AV_CHANNEL_LAYOUT_MASK(24, AV_CH_LAYOUT_22POINT2);

pub const AV_CHANNEL_LAYOUT_7POINT1_TOP_BACK: AVChannelLayout =
    AV_CHANNEL_LAYOUT_5POINT1POINT2_BACK;
