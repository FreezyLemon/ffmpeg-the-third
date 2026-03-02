use super::Context;
use crate::util::format;
use crate::{decoder, frame, Error};

use crate::ChannelLayout;

impl frame::Audio {
    #[inline]
    pub fn resampler2(
        &self,
        format: format::Sample,
        ch_layout: ChannelLayout,
        rate: u32,
    ) -> Result<Context, Error> {
        Context::get2(
            self.format(),
            self.ch_layout(),
            unsafe { (*self.as_ptr()).sample_rate as u32 },
            format,
            ch_layout,
            rate,
        )
    }
}

impl decoder::Audio {
    #[inline]
    pub fn resampler2(
        &self,
        format: format::Sample,
        ch_layout: ChannelLayout,
        rate: u32,
    ) -> Result<Context, Error> {
        Context::get2(
            self.format(),
            self.ch_layout(),
            self.rate(),
            format,
            ch_layout,
            rate,
        )
    }
}
