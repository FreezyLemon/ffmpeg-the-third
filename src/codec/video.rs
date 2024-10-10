use std::ops::Deref;

use super::codec::Codec;
use crate::codec::config::{ColorSpaceIter, ColorRangeIter};
use crate::codec::config::IterFromRef;

pub use crate::codec::config::FrameRateIter as RateIter;
pub use crate::codec::config::PixelFormatIter as FormatIter;

#[cfg(feature = "ffmpeg_7_1")]
use crate::codec::config::Supported;

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Video {
    codec: Codec,
}

impl Video {
    pub unsafe fn new(codec: Codec) -> Video {
        Video { codec }
    }
}

impl Video {
    /// Checks if the given frame rate is supported by this video codec.
    #[cfg(feature = "ffmpeg_7_1")]
    pub fn supports_rate(&self, rate: crate::Rational) -> bool {
        self.supported_rates().supports(rate)
    }

    /// Returns a [`Supported`] representing the supported frame rates.
    #[cfg(feature = "ffmpeg_7_1")]
    pub fn supported_rates(&self) -> Supported<RateIter> {
        use crate::codec::config::supported_frame_rates;

        supported_frame_rates(&self, None).expect("video codec returns supported frame rates")
    }

    pub fn rates(&self) -> Option<RateIter> {
        unsafe {
            (*self.codec.as_ptr())
                .supported_framerates
                .as_ref()
                .map(|fr| RateIter::from_ref(fr))
        }
    }

    /// Checks if the given pixel format is supported by this video codec.
    #[cfg(feature = "ffmpeg_7_1")]
    pub fn supports_format(&self, format: crate::format::Pixel) -> bool {
        self.supported_formats().supports(format)
    }

    /// Returns a [`Supported`] representing the supported pixel formats.
    #[cfg(feature = "ffmpeg_7_1")]
    pub fn supported_formats(&self) -> Supported<FormatIter> {
        use crate::codec::config::supported_pixel_formats;

        supported_pixel_formats(self, None).expect("video codec returns supported pixel formats")
    }

    pub fn formats(&self) -> Option<FormatIter> {
        unsafe {
            (*self.codec.as_ptr())
                .pix_fmts
                .as_ref()
                .map(|pf| FormatIter::from_ref(pf))
        }
    }

    /// Checks if the given color space is supported by this video codec.
    #[cfg(feature = "ffmpeg_7_1")]
    pub fn supports_color_space(&self, space: crate::color::Space) -> bool {
        self.supported_color_spaces().supports(space)
    }

    /// Returns a [`Supported`] representing the supported color spaces.
    #[cfg(feature = "ffmpeg_7_1")]
    pub fn supported_color_spaces(&self) -> Supported<ColorSpaceIter> {
        use crate::codec::config::supported_color_spaces;

        supported_color_spaces(self, None).expect("video codec returns supported color spaces")
    }

    /// Checks if the given color range is supported by this video codec.
    #[cfg(feature = "ffmpeg_7_1")]
    pub fn supports_color_range(&self, range: crate::color::Range) -> bool {
        self.supported_color_ranges().supports(range)
    }

    /// Returns a [`Supported`] representing the supported color ranges.
    #[cfg(feature = "ffmpeg_7_1")]
    pub fn supported_color_ranges(&self) -> Supported<ColorRangeIter> {
        use crate::codec::config::supported_color_ranges;

        supported_color_ranges(self, None).expect("video codec returns supported color ranges")
    }
}

impl Deref for Video {
    type Target = Codec;

    fn deref(&self) -> &Self::Target {
        &self.codec
    }
}
