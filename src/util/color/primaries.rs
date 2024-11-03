use crate::ffi::*;
use crate::utils;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Primaries {
    Reserved0,
    BT709,
    Unspecified,
    Reserved,
    BT470M,

    BT470BG,
    SMPTE170M,
    SMPTE240M,
    Film,
    BT2020,

    SMPTE428,
    SMPTE431,
    SMPTE432,
    #[cfg(not(feature = "ffmpeg_4_3"))]
    JEDEC_P22,
    #[cfg(feature = "ffmpeg_4_3")]
    EBU3213,
}

impl Primaries {
    #[cfg(feature = "ffmpeg_4_3")]
    pub const JEDEC_P22: Primaries = Primaries::EBU3213;

    pub fn name(&self) -> Option<&'static str> {
        if *self == Primaries::Unspecified {
            return None;
        }
        unsafe {
            let ptr = av_color_primaries_name((*self).into());
            utils::optional_str_from_c_ptr(ptr)
        }
    }
}

impl From<AVColorPrimaries> for Primaries {
    fn from(value: AVColorPrimaries) -> Primaries {
        use AVColorPrimaries as AV;

        match value {
            AV::AVCOL_PRI_RESERVED0 => Self::Reserved0,
            AV::AVCOL_PRI_BT709 => Self::BT709,
            AV::AVCOL_PRI_UNSPECIFIED => Self::Unspecified,
            AV::AVCOL_PRI_RESERVED => Self::Reserved,
            AV::AVCOL_PRI_BT470M => Self::BT470M,

            AV::AVCOL_PRI_BT470BG => Self::BT470BG,
            AV::AVCOL_PRI_SMPTE170M => Self::SMPTE170M,
            AV::AVCOL_PRI_SMPTE240M => Self::SMPTE240M,
            AV::AVCOL_PRI_FILM => Self::Film,
            AV::AVCOL_PRI_BT2020 => Self::BT2020,
            AV::AVCOL_PRI_NB => Self::Reserved0,

            AV::AVCOL_PRI_SMPTE428 => Self::SMPTE428,
            AV::AVCOL_PRI_SMPTE431 => Self::SMPTE431,
            AV::AVCOL_PRI_SMPTE432 => Self::SMPTE432,
            #[cfg(not(feature = "ffmpeg_4_3"))]
            AV::AVCOL_PRI_JEDEC_P22 => Self::JEDEC_P22,
            #[cfg(feature = "ffmpeg_4_3")]
            AV::AVCOL_PRI_EBU3213 => Self::EBU3213,

            #[cfg(feature = "non-exhaustive-enums")]
            _ => unimplemented!(),
        }
    }
}

impl From<Primaries> for AVColorPrimaries {
    fn from(value: Primaries) -> AVColorPrimaries {
        use Primaries as P;

        match value {
            P::Reserved0 => Self::AVCOL_PRI_RESERVED0,
            P::BT709 => Self::AVCOL_PRI_BT709,
            P::Unspecified => Self::AVCOL_PRI_UNSPECIFIED,
            P::Reserved => Self::AVCOL_PRI_RESERVED,
            P::BT470M => Self::AVCOL_PRI_BT470M,

            P::BT470BG => Self::AVCOL_PRI_BT470BG,
            P::SMPTE170M => Self::AVCOL_PRI_SMPTE170M,
            P::SMPTE240M => Self::AVCOL_PRI_SMPTE240M,
            P::Film => Self::AVCOL_PRI_FILM,
            P::BT2020 => Self::AVCOL_PRI_BT2020,

            P::SMPTE428 => Self::AVCOL_PRI_SMPTE428,
            P::SMPTE431 => Self::AVCOL_PRI_SMPTE431,
            P::SMPTE432 => Self::AVCOL_PRI_SMPTE432,
            #[cfg(not(feature = "ffmpeg_4_3"))]
            P::JEDEC_P22 => Self::AVCOL_PRI_JEDEC_P22,
            #[cfg(feature = "ffmpeg_4_3")]
            P::EBU3213 => Self::AVCOL_PRI_EBU3213,
        }
    }
}
