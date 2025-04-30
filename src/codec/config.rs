#[cfg(feature = "ffmpeg_7_1")]
use std::ptr::NonNull;

#[cfg(feature = "ffmpeg_7_1")]
use crate::{ffi::*, AsPtr, Codec, Error};

use crate::iters::TerminatedPtrIter;

#[cfg(feature = "ffmpeg_7_1")]
#[derive(Debug, Clone)]
pub enum Supported<I> {
    All,
    Specific(I),
}

#[cfg(feature = "ffmpeg_7_1")]
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
    /// let supported = codec.supported_frame_rates();
    /// assert!(supported.all())
    /// ```
    pub fn all(&self) -> bool {
        matches!(self, Supported::All)
    }

    /// Check if a specific configuration value is supported.
    ///
    /// # Example
    ///
    /// ```
    /// use ffmpeg_the_third::codec::{decoder, Id};
    /// use ffmpeg_the_third::format::sample::{Sample, Type};
    ///
    /// let codec = decoder::find(Id::MP3)
    ///     .expect("Can find an MP3 decoder")
    ///     .audio()
    ///     .unwrap();
    ///
    /// let supported = codec.supported_sample_formats();
    /// assert!(supported.supports(Sample::F32(Type::Planar)));
    /// ```
    pub fn supports(self, t: T) -> bool {
        match self {
            Supported::All => true,
            Supported::Specific(mut iter) => iter.any(|elem| elem == t),
        }
    }
}

macro_rules! impl_config_iter {
    (
        fns: (
            impl_for: $codec_type:ty;

            $(#[$all_fn_meta:meta])*
            fn_all: $all_fn:ident;

            $(#[$single_fn_meta:meta])*
            fn_single: $single_fn:ident($arg_name:ident);
        ),
        $codec_cfg:expr,
        $iter:ident (
            ptr: *const $ptr_ty:ty;
            terminator: $terminator:expr;
            wrapped: $wrapped_ty:ty;
        )
    ) => {
        #[cfg(feature = "ffmpeg_7_1")]
        impl Codec<$codec_type> {
            $(#[$all_fn_meta])*
            pub fn $all_fn(self) -> Supported<$iter<'static>> {
                let raw_ptr = GetCodecConfig::from_codec(self)
                    .get($codec_cfg)
                    .expect("avcodec_get_supported_config does not error for this codec");

                // SAFETY:
                // We have made sure that it's OK to cast the returned void pointer
                // into the appropriate pointer type for the given AVCodecConfig.
                match NonNull::new(raw_ptr as *mut _) {
                    Some(ptr) => unsafe { Supported::Specific($iter::from_ptr(ptr)) },
                    None => Supported::All,
                }
            }

            $(#[$single_fn_meta])*
            pub fn $single_fn(self, $arg_name: $wrapped_ty) -> bool {
                self.$all_fn().supports($arg_name)
            }
        }

        #[derive(Debug, Clone)]
        pub struct $iter<'a> {
            next: std::ptr::NonNull<$ptr_ty>,
            _marker: std::marker::PhantomData<&'a $ptr_ty>,
        }

        impl<'a> TerminatedPtrIter<$ptr_ty, $wrapped_ty> for $iter<'a> {
            unsafe fn from_ptr(ptr: std::ptr::NonNull<$ptr_ty>) -> Self {
                Self {
                    next: ptr,
                    _marker: std::marker::PhantomData,
                }
            }
        }

        // We make sure that this is true by not incrementing self.ptr after the
        // terminator has been reached.
        impl<'a> std::iter::FusedIterator for $iter<'a> {}

        // TODO: Maybe add ExactSizeIterator? This would require using the out_num_configs
        //       parameter and storing it inside $iter. Not sure it's too important unless
        //       many people want to use .collect() or something else that benefits from
        //       ExactSizeIterator.

        impl<'a> Iterator for $iter<'a> {
            type Item = $wrapped_ty;

            fn next(&mut self) -> Option<Self::Item> {
                // SAFETY: The FFmpeg API guarantees that the pointer is safe to deref and
                //         increment until the terminator is reached.
                unsafe {
                    let curr = self.next.as_ptr();
                    if *curr == $terminator {
                        return None;
                    }

                    // TODO: Replace with the following if MSRV >= 1.80:
                    // self.next = NonNull::from(self.next).add(1).as_ref();
                    self.next = std::ptr::NonNull::new_unchecked(curr.add(1));

                    Some((*curr).into())
                }
            }
        }
    };
}

#[derive(Debug)]
#[cfg(feature = "ffmpeg_7_1")]
pub struct GetCodecConfig {
    codec: *const AVCodec,
    ctx: *const AVCodecContext,
}

#[cfg(feature = "ffmpeg_7_1")]
impl GetCodecConfig {
    pub fn from_codec<C: AsPtr<AVCodec>>(codec: C) -> Self {
        Self {
            codec: codec.as_ptr(),
            ctx: std::ptr::null(),
        }
    }

    pub fn from_context<C: AsPtr<AVCodecContext>>(ctx: C) -> Self {
        Self {
            codec: std::ptr::null(),
            ctx: ctx.as_ptr(),
        }
    }

    pub fn with_codec<C: AsPtr<AVCodec>>(mut self, codec: C) -> Self {
        self.codec = codec.as_ptr();
        self
    }

    pub fn with_context<C: AsPtr<AVCodecContext>>(mut self, ctx: C) -> Self {
        self.ctx = ctx.as_ptr();
        self
    }

    pub fn get(&self, cfg: AVCodecConfig) -> Result<*const libc::c_void, Error> {
        assert!(
            !self.ctx.is_null() || !self.codec.is_null(),
            "cannot call avcodec_get_supported_config with two null pointers"
        );

        let mut out_ptr: *const libc::c_void = std::ptr::null();

        let ret = unsafe {
            avcodec_get_supported_config(
                self.ctx,
                self.codec,
                cfg,
                0, // flags: unused as of 7.1, set to zero
                &mut out_ptr,
                std::ptr::null_mut(), // out_num_configs: optional, we don't support it currently
            )
        };

        if ret < 0 {
            Err(Error::from(ret))
        } else {
            Ok(out_ptr)
        }
    }
}

impl_config_iter!(
    fns: (
        impl_for: crate::codec::codec::VideoType;
        fn_all: supported_pixel_formats;
        fn_single: supports_pixel_format(format);
    ),
    crate::ffi::AVCodecConfig::AV_CODEC_CONFIG_PIX_FORMAT,
    PixelFormatIter (
        ptr: *const crate::ffi::AVPixelFormat;
        terminator: crate::ffi::AVPixelFormat::AV_PIX_FMT_NONE;
        wrapped: crate::format::Pixel;
    )
);

impl_config_iter!(
    fns: (
        impl_for: crate::codec::codec::VideoType;
        fn_all: supported_frame_rates;
        fn_single: supports_frame_rate(rate);
    ),
    crate::ffi::AVCodecConfig::AV_CODEC_CONFIG_FRAME_RATE,
    FrameRateIter (
        ptr: *const crate::ffi::AVRational;
        terminator: crate::ffi::AVRational { num: 0, den: 0 };
        wrapped: crate::Rational;
    )
);

impl_config_iter!(
    fns: (
        impl_for: crate::codec::codec::AudioType;
        fn_all: supported_sample_rates;
        fn_single: supports_sample_rate(rate);
    ),
    crate::ffi::AVCodecConfig::AV_CODEC_CONFIG_SAMPLE_RATE,
    SampleRateIter (
        ptr: *const libc::c_int;
        terminator: 0 as libc::c_int;
        wrapped: libc::c_int;
    )
);

impl_config_iter!(
    fns: (
        impl_for: crate::codec::codec::AudioType;
        fn_all: supported_sample_formats;
        fn_single: supports_sample_format(format);
    ),
    crate::ffi::AVCodecConfig::AV_CODEC_CONFIG_SAMPLE_FORMAT,
    SampleFormatIter (
        ptr: *const crate::ffi::AVSampleFormat;
        terminator: crate::ffi::AVSampleFormat::AV_SAMPLE_FMT_NONE;
        wrapped: crate::format::Sample;
    )
);

impl_config_iter!(
    fns: (
        impl_for: crate::codec::codec::VideoType;
        fn_all: supported_color_ranges;
        fn_single: supports_color_range(range);
    ),
    crate::ffi::AVCodecConfig::AV_CODEC_CONFIG_COLOR_RANGE,
    ColorRangeIter (
        ptr: *const crate::ffi::AVColorRange;
        terminator: crate::ffi::AVColorRange::AVCOL_RANGE_UNSPECIFIED;
        wrapped: crate::color::Range;
    )
);

impl_config_iter!(
    fns: (
        impl_for: crate::codec::codec::VideoType;
        fn_all: supported_color_spaces;
        fn_single: supports_color_space(space);
    ),
    crate::ffi::AVCodecConfig::AV_CODEC_CONFIG_COLOR_SPACE,
    ColorSpaceIter (
        ptr: *const crate::ffi::AVColorSpace;
        terminator: crate::ffi::AVColorSpace::AVCOL_SPC_UNSPECIFIED;
        wrapped: crate::color::Space;
    )
);

#[cfg(test)]
#[cfg(feature = "ffmpeg_7_1")]
mod test {
    use super::*;

    use crate::codec::{decoder, encoder, Id};
    use crate::color::Range;
    use crate::format::Pixel;
    use crate::Rational;

    // These tests can fail if the FFmpeg build does not contain the required de/encoder.
    // TODO: Check if tests can be hidden behind feature flags.

    #[test]
    fn audio_decoder() {
        let codec = decoder::find(Id::MP3)
            .and_then(|c| c.audio())
            .expect("can find mp3 audio decoder");

        // Audio decoder does not have color ranges
        assert!(GetCodecConfig::from_codec(codec)
            .get(AVCodecConfig::AV_CODEC_CONFIG_COLOR_RANGE)
            .is_err());

        let format_iter = match codec.supported_sample_formats() {
            Supported::Specific(f) => f,
            sup => panic!("Should be Supported::Specific, got {sup:#?}"),
        };

        for format in format_iter {
            println!("format: {format:#?}");
        }
    }

    #[test]
    fn audio_encoder() {
        let codec = encoder::find(Id::OPUS)
            .and_then(|c| c.audio())
            .expect("can find opus audio encoder");

        let format_iter = match codec.supported_sample_formats() {
            Supported::Specific(f) => f,
            sup => panic!("Should be Supported::Specific, got {sup:#?}"),
        };

        for format in format_iter {
            println!("format: {format:#?}");
        }
    }

    #[test]
    fn video_decoder() {
        let codec = decoder::find(Id::H264)
            .and_then(|c| c.video())
            .expect("can find H264 decoder");

        assert!(GetCodecConfig::from_codec(codec)
            .get(AVCodecConfig::AV_CODEC_CONFIG_SAMPLE_RATE)
            .is_err());
        assert!(matches!(codec.supported_color_spaces(), Supported::All));
    }

    #[test]
    fn video_encoder() {
        let codec = encoder::find(Id::VP9)
            .and_then(|c| c.video())
            .expect("can find VP9 video encoder");

        let color_ranges = match codec.supported_color_ranges() {
            Supported::Specific(c) => c,
            sup => panic!("Should be Supported::Specific, got {sup:#?}"),
        };

        for range in color_ranges {
            println!("{range:#?}");
        }

        assert!(matches!(
            codec.supported_pixel_formats(),
            Supported::Specific(_)
        ));

        assert!(matches!(codec.supported_frame_rates(), Supported::All));
    }

    #[test]
    fn supports() {
        let codec = encoder::find(Id::FFV1)
            .and_then(|c| c.video())
            .expect("can find FFV1 video encoder");

        assert!(codec.supported_color_ranges().supports(Range::MPEG));

        assert!(!codec.supported_pixel_formats().supports(Pixel::GRAY16));

        assert!(codec.supported_frame_rates().supports(Rational(123, 456)));

        GetCodecConfig::from_codec(codec)
            .get(AVCodecConfig::AV_CODEC_CONFIG_SAMPLE_FORMAT)
            .expect_err("can NOT check sample format support (video codec)");
    }

    // TODO: Reimplement after Context rework
    // #[test]
    // fn with_context() {
    //     let codec = encoder::find(Id::MJPEG).and_then(|c| c.video()).expect("can find MJPEG video encoder");

    //     let mut ctx = unsafe {
    //         let avctx = crate::ffi::avcodec_alloc_context3(codec.as_ptr());
    //         crate::codec::Context::wrap(avctx, None)
    //     };

    //     ctx.compliance(Compliance::Strict);

    //     assert!(!supported_color_ranges(ctx.codec().unwrap(), Some(&ctx))
    //         .expect("can check color range support")
    //         .supports(Range::MPEG));

    //     ctx.compliance(Compliance::Unofficial);

    //     // Note that we check for NOT supported above, and YES supported here
    //     // MJPEG encoder only supports MPEG color range if compliance is
    //     // Unofficial or lower (less strict)
    //     assert!(supported_color_ranges(ctx.codec().unwrap(), Some(&ctx))
    //         .expect("can check color range support")
    //         .supports(Range::MPEG));
    // }
}
