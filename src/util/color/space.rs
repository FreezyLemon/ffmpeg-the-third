use crate::ffi::*;
use crate::utils;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Space {
    RGB,
    BT709,
    Unspecified,
    Reserved,
    FCC,
    BT470BG,
    SMPTE170M,
    SMPTE240M,
    YCGCO,
    BT2020NCL,
    BT2020CL,
    SMPTE2085,

    ChromaDerivedNCL,
    ChromaDerivedCL,
    ICTCP,

    #[cfg(feature = "ffmpeg_7_1")]
    IPTC2,
    #[cfg(feature = "ffmpeg_7_1")]
    YCGCORE,
    #[cfg(feature = "ffmpeg_7_1")]
    YCGCORO,
}

impl Space {
    pub const YCOCG: Space = Space::YCGCO;

    pub fn name(&self) -> Option<&'static str> {
        if *self == Space::Unspecified {
            return None;
        }
        unsafe {
            let ptr = av_color_space_name((*self).into());
            utils::optional_str_from_c_ptr(ptr)
        }
    }
}

impl From<AVColorSpace> for Space {
    fn from(value: AVColorSpace) -> Self {
        use AVColorSpace as AV;

        match value {
            AV::AVCOL_SPC_RGB => Self::RGB,
            AV::AVCOL_SPC_BT709 => Self::BT709,
            AV::AVCOL_SPC_UNSPECIFIED => Self::Unspecified,
            AV::AVCOL_SPC_RESERVED => Self::Reserved,
            AV::AVCOL_SPC_FCC => Self::FCC,
            AV::AVCOL_SPC_BT470BG => Self::BT470BG,
            AV::AVCOL_SPC_SMPTE170M => Self::SMPTE170M,
            AV::AVCOL_SPC_SMPTE240M => Self::SMPTE240M,
            AV::AVCOL_SPC_YCGCO => Self::YCGCO,
            AV::AVCOL_SPC_BT2020_NCL => Self::BT2020NCL,
            AV::AVCOL_SPC_BT2020_CL => Self::BT2020CL,
            AV::AVCOL_SPC_SMPTE2085 => Self::SMPTE2085,
            AV::AVCOL_SPC_NB => Self::Unspecified,

            AV::AVCOL_SPC_CHROMA_DERIVED_NCL => Self::ChromaDerivedNCL,
            AV::AVCOL_SPC_CHROMA_DERIVED_CL => Self::ChromaDerivedCL,
            AV::AVCOL_SPC_ICTCP => Self::ICTCP,

            #[cfg(feature = "ffmpeg_7_1")]
            AV::AVCOL_SPC_IPT_C2 => Self::IPTC2,
            #[cfg(feature = "ffmpeg_7_1")]
            AV::AVCOL_SPC_YCGCO_RE => Self::YCGCORE,
            #[cfg(feature = "ffmpeg_7_1")]
            AV::AVCOL_SPC_YCGCO_RO => Self::YCGCORO,

            #[cfg(feature = "non-exhaustive-enums")]
            _ => unimplemented!(),
        }
    }
}

impl From<Space> for AVColorSpace {
    fn from(value: Space) -> AVColorSpace {
        use Space as S;

        match value {
            S::RGB => Self::AVCOL_SPC_RGB,
            S::BT709 => Self::AVCOL_SPC_BT709,
            S::Unspecified => Self::AVCOL_SPC_UNSPECIFIED,
            S::Reserved => Self::AVCOL_SPC_RESERVED,
            S::FCC => Self::AVCOL_SPC_FCC,
            S::BT470BG => Self::AVCOL_SPC_BT470BG,
            S::SMPTE170M => Self::AVCOL_SPC_SMPTE170M,
            S::SMPTE240M => Self::AVCOL_SPC_SMPTE240M,
            S::YCGCO => Self::AVCOL_SPC_YCGCO,
            S::BT2020NCL => Self::AVCOL_SPC_BT2020_NCL,
            S::BT2020CL => Self::AVCOL_SPC_BT2020_CL,
            S::SMPTE2085 => Self::AVCOL_SPC_SMPTE2085,

            S::ChromaDerivedNCL => Self::AVCOL_SPC_CHROMA_DERIVED_NCL,
            S::ChromaDerivedCL => Self::AVCOL_SPC_CHROMA_DERIVED_CL,
            S::ICTCP => Self::AVCOL_SPC_ICTCP,

            #[cfg(feature = "ffmpeg_7_1")]
            S::IPTC2 => Self::AVCOL_SPC_IPT_C2,
            #[cfg(feature = "ffmpeg_7_1")]
            S::YCGCORE => Self::AVCOL_SPC_YCGCO_RE,
            #[cfg(feature = "ffmpeg_7_1")]
            S::YCGCORO => Self::AVCOL_SPC_YCGCO_RO,
        }
    }
}
