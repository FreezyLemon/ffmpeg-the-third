use crate::ffi::*;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Dither {
    None,
    Rectangular,
    Triangular,
    TriangularHighPass,

    NoiseShapingLipshitz,
    NoiseShapingFWeighted,
    NoiseShapingModifiedEWeighted,
    NoiseShapingImprovedEWeighted,
    NoiseShapingShibata,
    NoiseShapingLowShibata,
    NoiseShapingHighShibata,
}

impl From<SwrDitherType> for Dither {
    fn from(value: SwrDitherType) -> Dither {
        use SwrDitherType as AV;

        match value {
            AV::SWR_DITHER_NONE => Dither::None,
            AV::SWR_DITHER_RECTANGULAR => Dither::Rectangular,
            AV::SWR_DITHER_TRIANGULAR => Dither::Triangular,
            AV::SWR_DITHER_TRIANGULAR_HIGHPASS => Dither::TriangularHighPass,

            AV::SWR_DITHER_NS => Dither::None,
            AV::SWR_DITHER_NS_LIPSHITZ => Dither::NoiseShapingLipshitz,
            AV::SWR_DITHER_NS_F_WEIGHTED => Dither::NoiseShapingFWeighted,
            AV::SWR_DITHER_NS_MODIFIED_E_WEIGHTED => Dither::NoiseShapingModifiedEWeighted,
            AV::SWR_DITHER_NS_IMPROVED_E_WEIGHTED => Dither::NoiseShapingImprovedEWeighted,
            AV::SWR_DITHER_NS_SHIBATA => Dither::NoiseShapingShibata,
            AV::SWR_DITHER_NS_LOW_SHIBATA => Dither::NoiseShapingLowShibata,
            AV::SWR_DITHER_NS_HIGH_SHIBATA => Dither::NoiseShapingHighShibata,
            AV::SWR_DITHER_NB => Dither::None,

            #[cfg(feature = "non-exhaustive-enums")]
            _ => unimplemented!(),
        }
    }
}

impl From<Dither> for SwrDitherType {
    fn from(value: Dither) -> SwrDitherType {
        match value {
            Dither::None => Self::SWR_DITHER_NONE,
            Dither::Rectangular => Self::SWR_DITHER_RECTANGULAR,
            Dither::Triangular => Self::SWR_DITHER_TRIANGULAR,
            Dither::TriangularHighPass => Self::SWR_DITHER_TRIANGULAR_HIGHPASS,

            Dither::NoiseShapingLipshitz => Self::SWR_DITHER_NS_LIPSHITZ,
            Dither::NoiseShapingFWeighted => Self::SWR_DITHER_NS_F_WEIGHTED,
            Dither::NoiseShapingModifiedEWeighted => Self::SWR_DITHER_NS_MODIFIED_E_WEIGHTED,
            Dither::NoiseShapingImprovedEWeighted => Self::SWR_DITHER_NS_IMPROVED_E_WEIGHTED,
            Dither::NoiseShapingShibata => Self::SWR_DITHER_NS_SHIBATA,
            Dither::NoiseShapingLowShibata => Self::SWR_DITHER_NS_LOW_SHIBATA,
            Dither::NoiseShapingHighShibata => Self::SWR_DITHER_NS_HIGH_SHIBATA,
        }
    }
}
