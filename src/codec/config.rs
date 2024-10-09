use crate::codec::Context;
use crate::ffi::*;
use crate::Codec;
use crate::Error;

use libc::{c_int, c_void};

#[derive(Debug, Clone)]
pub enum Supported<T, I: Iterator<Item = T>> {
    All,
    Specific(I),
    Error(crate::Error),
}

fn supported<T, I, F>(
    codec: Codec,
    ctx: Option<Context>,
    cfg: AVCodecConfig,
    create_iter: F,
) -> Supported<T, I>
where
    I: Iterator<Item = T>,
    F: FnOnce(*const c_void) -> I,
{
    let mut out_ptr: *const c_void = std::ptr::null();
    let mut out_num = 0 as c_int;

    let ret = unsafe {
        let avctx = ctx.map_or(std::ptr::null(), |ctx| ctx.as_ptr());

        avcodec_get_supported_config(
            avctx,
            codec.as_ptr(),
            cfg,
            0, // flags: currently unused (as of 7.1), set to zero
            &mut out_ptr,
            &mut out_num,
        )
    };

    if ret < 0 {
        Supported::Error(Error::from(ret))
    } else if out_ptr.is_null() {
        Supported::All
    } else {
        Supported::Specific(create_iter(out_ptr))
    }
}

macro_rules! impl_config_iter {
    ($fn_name:ident, $codec_cfg:expr, $iter:ident, $ty:ty, $av_ty:ty, $terminator:expr) => {
        pub(crate) fn $fn_name(codec: Codec, ctx: Option<Context>) -> Supported<$ty, $iter> {
            supported(codec, ctx, $codec_cfg, |ptr| $iter {
                curr: ptr as *const $av_ty,
            })
        }

        #[derive(Debug, Clone)]
        pub struct $iter {
            curr: *const $av_ty,
        }

        // We make sure that this is true by not incrementing self.curr after the
        // terminator has been reached. The implementation also handles NULL ptrs.
        impl std::iter::FusedIterator for $iter {}

        // No ExactSizeIterator even though FFmpeg supports returning out_num,
        // because the FFmpeg implementation seems to be fairly inefficient, at
        // least in some cases.

        impl Iterator for $iter {
            type Item = $ty;

            fn next(&mut self) -> Option<Self::Item> {
                if self.curr.is_null() {
                    // This should not usually happen, but at the very least we want to make sure to
                    // not deref a null ptr.
                    return None;
                }

                // SAFETY: The FFmpeg API guarantees that the pointer is either NULL
                // or safe to deref & increment until the terminator is reached.
                unsafe {
                    let val = *self.curr;
                    if val != $terminator {
                        self.curr = self.curr.add(1);
                        Some(<$ty>::from(val))
                    } else {
                        None
                    }
                }
            }
        }
    };
}

impl_config_iter!(
    supported_pixel_formats,
    crate::ffi::AVCodecConfig::AV_CODEC_CONFIG_PIX_FORMAT,
    PixelFormatIter,
    crate::format::Pixel,
    crate::ffi::AVPixelFormat,
    crate::ffi::AVPixelFormat::AV_PIX_FMT_NONE
);

impl_config_iter!(
    supported_frame_rates,
    crate::ffi::AVCodecConfig::AV_CODEC_CONFIG_FRAME_RATE,
    FrameRateIter,
    crate::Rational,
    crate::ffi::AVRational,
    crate::ffi::AVRational { num: 0, den: 0 }
);

impl_config_iter!(
    supported_sample_rates,
    crate::ffi::AVCodecConfig::AV_CODEC_CONFIG_SAMPLE_RATE,
    SampleRateIter,
    libc::c_int,
    libc::c_int,
    0 as libc::c_int
);

impl_config_iter!(
    supported_sample_formats,
    crate::ffi::AVCodecConfig::AV_CODEC_CONFIG_SAMPLE_FORMAT,
    SampleFormatIter,
    crate::format::Sample,
    crate::ffi::AVSampleFormat,
    crate::ffi::AVSampleFormat::AV_SAMPLE_FMT_NONE
);

// TODO: is 'static correct?
// impl_config_iter!(
//     supported_channel_layouts,
//     crate::ffi::AVCodecConfig::AV_CODEC_CONFIG_CH_LAYOUTS,
//     ChannelLayoutIter,
//     crate::ChannelLayout<'static>,
//     crate::ffi::AVChannelLayout,
//     crate::ffi::AVChannelLayout::empty()
// );

impl_config_iter!(
    supported_color_ranges,
    crate::ffi::AVCodecConfig::AV_CODEC_CONFIG_COLOR_RANGE,
    ColorRangeIter,
    crate::color::Range,
    crate::ffi::AVColorRange,
    crate::ffi::AVColorRange::AVCOL_RANGE_UNSPECIFIED
);

impl_config_iter!(
    supported_color_spaces,
    crate::ffi::AVCodecConfig::AV_CODEC_CONFIG_COLOR_SPACE,
    ColorSpaceIter,
    crate::color::Space,
    crate::ffi::AVColorSpace,
    crate::ffi::AVColorSpace::AVCOL_SPC_UNSPECIFIED
);

#[cfg(test)]
mod test {
    use super::*;

    use crate::codec::Id;
    use crate::codec::{decoder, encoder};
    
    // Thes tests can fail if the FFmpeg build does not contain the required de/encoder

    #[test]
    fn audio_decoder() {
        let codec = decoder::find(Id::MP3).expect("can find mp3 decoder");

        // Audio decoder does not have color ranges
        assert!(matches!(supported_color_ranges(codec, None), Supported::Error(_)));

        let format_iter = match supported_sample_formats(codec, None) {
            Supported::Specific(f) => f,
            sup => panic!("Should be Supported::Specific, got {sup:?}"),
        };

        for format in format_iter {
            println!("{format:?}");
        }
    }

    #[test]
    fn video_decoder() {
        let codec = decoder::find(Id::H264).expect("can find H264 decoder");

        assert!(matches!(supported_sample_rates(codec, None), Supported::Error(_)));
        assert!(matches!(supported_color_spaces(codec, None), Supported::All));
    }
}
