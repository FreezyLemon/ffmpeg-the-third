use crate::ffi::*;
use crate::utils;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Range {
    Unspecified,
    MPEG,
    JPEG,
}

impl Range {
    pub fn name(&self) -> Option<&'static str> {
        if *self == Range::Unspecified {
            return None;
        }
        unsafe {
            let ptr = av_color_range_name((*self).into());
            utils::optional_str_from_c_ptr(ptr)
        }
    }
}

impl From<AVColorRange> for Range {
    fn from(value: AVColorRange) -> Self {
        use AVColorRange as AV;

        match value {
            AV::AVCOL_RANGE_UNSPECIFIED => Self::Unspecified,
            AV::AVCOL_RANGE_MPEG => Self::MPEG,
            AV::AVCOL_RANGE_JPEG => Self::JPEG,
            AV::AVCOL_RANGE_NB => Self::Unspecified,

            #[cfg(feature = "non-exhaustive-enums")]
            _ => unimplemented!(),
        }
    }
}

impl From<Range> for AVColorRange {
    fn from(value: Range) -> AVColorRange {
        use Range as R;

        match value {
            R::Unspecified => Self::AVCOL_RANGE_UNSPECIFIED,
            R::MPEG => Self::AVCOL_RANGE_MPEG,
            R::JPEG => Self::AVCOL_RANGE_JPEG,
        }
    }
}
