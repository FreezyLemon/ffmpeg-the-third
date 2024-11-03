use crate::ffi::*;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Type {
    None,
    I,
    P,
    B,
    S,
    SI,
    SP,
    BI,
}

impl From<AVPictureType> for Type {
    #[inline(always)]
    fn from(value: AVPictureType) -> Type {
        use AVPictureType as AV;

        match value {
            AV::AV_PICTURE_TYPE_NONE => Type::None,
            AV::AV_PICTURE_TYPE_I => Type::I,
            AV::AV_PICTURE_TYPE_P => Type::P,
            AV::AV_PICTURE_TYPE_B => Type::B,
            AV::AV_PICTURE_TYPE_S => Type::S,
            AV::AV_PICTURE_TYPE_SI => Type::SI,
            AV::AV_PICTURE_TYPE_SP => Type::SP,
            AV::AV_PICTURE_TYPE_BI => Type::BI,

            #[cfg(feature = "non-exhaustive-enums")]
            _ => unimplemented!(),
        }
    }
}

impl From<Type> for AVPictureType {
    #[inline(always)]
    fn from(value: Type) -> AVPictureType {
        match value {
            Type::None => Self::AV_PICTURE_TYPE_NONE,
            Type::I => Self::AV_PICTURE_TYPE_I,
            Type::P => Self::AV_PICTURE_TYPE_P,
            Type::B => Self::AV_PICTURE_TYPE_B,
            Type::S => Self::AV_PICTURE_TYPE_S,
            Type::SI => Self::AV_PICTURE_TYPE_SI,
            Type::SP => Self::AV_PICTURE_TYPE_SP,
            Type::BI => Self::AV_PICTURE_TYPE_BI,
        }
    }
}
