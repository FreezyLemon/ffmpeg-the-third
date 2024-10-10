use crate::codec::Context;
use crate::ffi::*;
use crate::Codec;
use crate::Error;

use libc::c_void;

#[derive(Debug, Clone)]
pub enum Supported<I> {
    All,
    Specific(I),
}

impl<T, I> Supported<I>
where
    T: PartialEq,
    I: Iterator<Item = T>,
{
    /// Check if all possible configuration values are supported.
    ///
    /// # Example
    ///
    /// ```
    /// use ffmpeg_the_third::codec::{encoder, Id};
    ///
    /// let codec = encoder::find(Id::VP9)
    ///     .expect("Can find a VP9 encoder")
    ///     .video()
    ///     .unwrap();
    ///
    /// let supported = codec.supported_rates();
    /// assert!(supported.supports_all())
    /// ```
    pub fn supports_all(&self) -> bool {
        matches!(self, Supported::All)
    }

    /// Check if a specific configuration value is supported.
    ///
    /// # Example
    ///
    /// ```
    /// use ffmpeg_the_third::codec::{encoder, Id};
    /// use ffmpeg_the_third::format::sample::{Sample, Type};
    ///
    /// let codec = encoder::find(Id::MP3)
    ///     .expect("Can find an MP3 encoder")
    ///     .audio()
    ///     .unwrap();
    ///
    /// let supported = codec.supported_formats();
    /// assert!(supported.supports(Sample::F32(Type::Planar)));
    /// ```
    pub fn supports(self, t: T) -> bool {
        match self {
            Supported::All => true,
            Supported::Specific(mut iter) => iter.any(|elem| elem == t),
        }
    }
}

#[cfg(feature = "ffmpeg_7_1")]
fn supported<'codec, WrapperType, AVType, I>(
    codec: &'codec Codec,
    ctx: Option<&Context>,
    cfg: AVCodecConfig,
) -> Result<Supported<I>, Error>
where
    I: Iterator<Item = WrapperType> + IterFromRef<'codec, AVType>,
    AVType: 'codec + Into<WrapperType>,
{
    let mut out_ptr: *const c_void = std::ptr::null();

    unsafe {
        let avctx = ctx.map_or(std::ptr::null(), |ctx| ctx.as_ptr());

        let ret = avcodec_get_supported_config(
            avctx,
            codec.as_ptr(),
            cfg,
            0, // flags: unused as of 7.1, set to zero
            &mut out_ptr,
            std::ptr::null_mut(), // out_num_configs: optional, we don't support it currently
        );

        if ret < 0 {
            Err(Error::from(ret))
        } else {
            // SAFETY: FFmpeg returns a list of configs via out_ptr, so we trust that it
            //         is safe to dereference and hold a reference that lives as long as
            //         the codec used to get the config value.
            match (out_ptr as *const AVType).as_ref::<'codec>() {
                // nullptr -> Everything is supported
                None => Ok(Supported::All),
                // non-nullptr -> Specific list of values is supported.
                // Since the iterator holds a reference, we know that the iterator too only
                // lives as long as the reference (= 'codec).
                Some(r) => Ok(Supported::Specific(I::from_ref(r))),
            }
        }
    }
}

// Similar to core::convert::From, but with a function marked `unsafe` because there are some
// special conditions to make it safe.
pub(crate) trait IterFromRef<'a, T>: Sized {
    /// Create an iterator from a reference to the first element in the iteration.
    /// This is used internally to implement iterators based on the FFmpeg APIs.
    ///
    /// # Safety
    ///
    /// This results in undefined behavior when it is not safe to iterate through references
    /// via pointer addition (C-style list) like
    ///
    /// ```ignore
    /// next = next.as_ptr().add(1).as_ref().unwrap()
    /// ```
    ///
    /// until the type-specific terminator is reached.
    unsafe fn from_ref(next: &'a T) -> Self;
}

macro_rules! impl_config_iter_struct {
    ($iter:ident, $av_ty:ty) => {
        #[derive(Debug, Clone)]
        pub struct $iter<'a> {
            next: &'a $av_ty,
        }

        impl<'a> IterFromRef<'a, $av_ty> for $iter<'a> {
            unsafe fn from_ref(next: &'a $av_ty) -> Self {
                Self { next }
            }
        }
    };
}

macro_rules! impl_config_iter_fn {
    ($fn_name:ident, $iter:ident, $codec_cfg:expr) => {
        /// Low-level function interacting with the FFmpeg API via
        /// `avcodec_get_supported_config()`. Consider using one of the convenience methods
        /// on the codecs or codec contexts instead.
        #[cfg(feature = "ffmpeg_7_1")]
        pub fn $fn_name<'codec>(
            codec: &'codec Codec,
            ctx: Option<&Context>,
        ) -> Result<Supported<$iter<'codec>>, Error> {
            supported(codec, ctx, $codec_cfg)
        }
    };
}

macro_rules! impl_config_iter_traits {
    ($iter:ident, $ty:ty, $av_ty:ty, $terminator:expr) => {
        impl_config_iter_traits_inner!($iter, $ty, $av_ty, $terminator, true);
    };
    (keep_ref, $iter:ident, $ty:ty, $av_ty:ty, $terminator:expr) => {
        impl_config_iter_traits_inner!($iter, $ty, $av_ty, $terminator);
    };
}

macro_rules! impl_config_iter_traits_inner {
    ($iter:ident, $ty:ty, $av_ty:ty, $terminator:expr $(, $deref:literal)?) => {
        // We make sure that this is true by not incrementing self.ptr after the
        // terminator has been reached.
        impl<'a> std::iter::FusedIterator for $iter<'a> {}

        // TODO: Maybe add ExactSizeIterator? This would require using the out_num_configs
        //       parameter and storing it inside $iter. Not sure it's too important unless
        //       many people want to use .collect() or something else that benefits from
        //       ExactSizeIterator.

        impl<'a> Iterator for $iter<'a> {
            type Item = $ty;

            fn next(&mut self) -> Option<Self::Item> {
                // SAFETY: The FFmpeg API guarantees that the pointer is safe to deref and
                //         increment until the terminator is reached.
                unsafe {
                    let curr = self.next;
                    if *curr == $terminator {
                        return None;
                    }

                    // TODO: Replace with the following if MSRV >= 1.80:
                    // self.next = NonNull::from(self.next).add(1).as_ref();
                    self.next = (self.next as *const $av_ty).add(1).as_ref().unwrap();

                    $(
                        { $deref };
                        let curr = *curr;
                    )?
                    Some(curr.into())
                }
            }
        }
    };
}

macro_rules! impl_config_iter {
    (
        keep_ref,
        $fn_name:ident,
        $codec_cfg:expr,
        $iter:ident,
        $ty:ty,
        $av_ty:ty,
        $terminator:expr
    ) => {
        impl_config_iter_fn!($fn_name, $iter, $codec_cfg);
        impl_config_iter_struct!($iter, $av_ty);
        impl_config_iter_traits!(keep_ref, $iter, $ty, $av_ty, $terminator);
    };
    (
        $fn_name:ident,
        $codec_cfg:expr,
        $iter:ident,
        $ty:ty,
        $av_ty:ty,
        $terminator:expr
    ) => {
        impl_config_iter_fn!($fn_name, $iter, $codec_cfg);
        impl_config_iter_struct!($iter, $av_ty);
        impl_config_iter_traits!($iter, $ty, $av_ty, $terminator);
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

#[cfg(feature = "ffmpeg_7_1")]
impl_config_iter!(
    supported_color_ranges,
    crate::ffi::AVCodecConfig::AV_CODEC_CONFIG_COLOR_RANGE,
    ColorRangeIter,
    crate::color::Range,
    crate::ffi::AVColorRange,
    crate::ffi::AVColorRange::AVCOL_RANGE_UNSPECIFIED
);

#[cfg(feature = "ffmpeg_7_1")]
impl_config_iter!(
    supported_color_spaces,
    crate::ffi::AVCodecConfig::AV_CODEC_CONFIG_COLOR_SPACE,
    ColorSpaceIter,
    crate::color::Space,
    crate::ffi::AVColorSpace,
    crate::ffi::AVColorSpace::AVCOL_SPC_UNSPECIFIED
);

#[cfg(feature = "ffmpeg_5_1")]
impl_config_iter!(
    keep_ref,
    supported_channel_layouts,
    crate::ffi::AVCodecConfig::AV_CODEC_CONFIG_CHANNEL_LAYOUT,
    ChannelLayoutIter,
    crate::ChannelLayout<'a>,
    crate::ffi::AVChannelLayout,
    std::mem::zeroed()
);

#[cfg(test)]
mod test {
    use super::*;

    use crate::codec::{decoder, encoder, Compliance, Id};
    use crate::color::Range;
    use crate::format::Pixel;
    use crate::Rational;

    // These tests can fail if the FFmpeg build does not contain the required de/encoder.
    // TODO: Check if tests can be hidden behind feature flags or something.

    #[test]
    fn audio_decoder() {
        let codec = decoder::find(Id::MP3).expect("can find mp3 decoder");

        // Audio decoder does not have color ranges
        assert!(supported_color_ranges(&codec, None).is_err());

        let format_iter = match supported_sample_formats(&codec, None) {
            Ok(Supported::Specific(f)) => f,
            sup => panic!("Should be Supported::Specific, got {sup:#?}"),
        };

        for format in format_iter {
            println!("format: {format:#?}");
        }

        assert!(matches!(
            supported_channel_layouts(&codec, None),
            Ok(Supported::All)
        ));
    }

    #[test]
    fn audio_encoder() {
        let codec = encoder::find(Id::MP3).expect("can find mp3 encoder");

        // looks like every codec returns Supported::All for color space.
        // might change in a future FFmpeg release
        assert!(matches!(
            supported_color_spaces(&codec, None),
            Ok(Supported::All)
        ));
        let format_iter = match supported_sample_formats(&codec, None) {
            Ok(Supported::Specific(f)) => f,
            sup => panic!("Should be Supported::Specific, got {sup:#?}"),
        };

        for format in format_iter {
            println!("format: {format:#?}");
        }

        let layout_iter = match supported_channel_layouts(&codec, None) {
            Ok(Supported::Specific(l)) => l,
            sup => panic!("Should be Supported::Specific, got {sup:#?}"),
        };

        for layout in layout_iter {
            println!("layout: {layout:#?}");
        }
    }

    #[test]
    fn video_decoder() {
        let codec = decoder::find(Id::H264).expect("can find H264 decoder");

        assert!(supported_sample_rates(&codec, None).is_err());
        assert!(matches!(
            supported_color_spaces(&codec, None),
            Ok(Supported::All)
        ));
    }

    #[test]
    fn video_encoder() {
        let codec = encoder::find(Id::VP9).expect("can find VP9 encoder");

        let color_ranges = match supported_color_ranges(&codec, None) {
            Ok(Supported::Specific(c)) => c,
            sup => panic!("Should be Supported::Specific, got {sup:#?}"),
        };

        for range in color_ranges {
            println!("{range:#?}");
        }

        assert!(matches!(
            supported_pixel_formats(&codec, None),
            Ok(Supported::Specific(_))
        ));

        assert!(matches!(
            supported_frame_rates(&codec, None),
            Ok(Supported::All)
        ));
    }

    #[test]
    fn supports() {
        let codec = encoder::find(Id::VP9).expect("can find VP9 encoder");

        assert!(supported_color_ranges(&codec, None)
            .expect("can check color range support")
            .supports(Range::JPEG));

        assert!(!supported_pixel_formats(&codec, None)
            .expect("can check color range support")
            .supports(Pixel::BGR8));

        assert!(supported_frame_rates(&codec, None)
            .expect("can check frame rate support")
            .supports(Rational(123, 456)));

        supported_sample_formats(&codec, None)
            .expect_err("can NOT check sample format support (video codec)");
    }

    #[test]
    fn with_context() {
        let codec = encoder::find(Id::MJPEG).expect("can find MJPEG encoder");

        let mut ctx = unsafe {
            let avctx = avcodec_alloc_context3(codec.as_ptr());
            Context::wrap(avctx, None)
        };

        ctx.compliance(Compliance::Strict);

        assert!(!supported_color_ranges(&ctx.codec().unwrap(), Some(&ctx))
            .expect("can check color range support")
            .supports(Range::MPEG));

        ctx.compliance(Compliance::Unofficial);

        // Note that we check for NOT supported above, and YES supported here
        // MJPEG encoder only supports MPEG color range if compliance is
        // Unofficial or lower (less strict)
        assert!(supported_color_ranges(&ctx.codec().unwrap(), Some(&ctx))
            .expect("can check color range support")
            .supports(Range::MPEG));
    }
}
