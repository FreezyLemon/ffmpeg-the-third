use crate::ffi::*;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Rounding {
    Zero,
    Infinity,
    Down,
    Up,
    NearInfinity,
    PassMinMax,
}

impl From<AVRounding> for Rounding {
    #[inline(always)]
    fn from(value: AVRounding) -> Self {
        use AVRounding as AV;

        match value {
            AV::AV_ROUND_ZERO => Rounding::Zero,
            AV::AV_ROUND_INF => Rounding::Infinity,
            AV::AV_ROUND_DOWN => Rounding::Down,
            AV::AV_ROUND_UP => Rounding::Up,
            AV::AV_ROUND_NEAR_INF => Rounding::NearInfinity,
            AV::AV_ROUND_PASS_MINMAX => Rounding::PassMinMax,

            #[cfg(feature = "non-exhaustive-enums")]
            _ => unimplemented!(),
        }
    }
}

impl From<Rounding> for AVRounding {
    #[inline(always)]
    fn from(value: Rounding) -> AVRounding {
        match value {
            Rounding::Zero => Self::AV_ROUND_ZERO,
            Rounding::Infinity => Self::AV_ROUND_INF,
            Rounding::Down => Self::AV_ROUND_DOWN,
            Rounding::Up => Self::AV_ROUND_UP,
            Rounding::NearInfinity => Self::AV_ROUND_NEAR_INF,
            Rounding::PassMinMax => Self::AV_ROUND_PASS_MINMAX,
        }
    }
}
