use crate::ffi::*;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Type {
    Unknown,
    Video,
    Audio,
    Data,
    Subtitle,
    Attachment,
}

impl From<AVMediaType> for Type {
    #[inline(always)]
    fn from(value: AVMediaType) -> Self {
        use AVMediaType as AV;

        match value {
            AV::AVMEDIA_TYPE_UNKNOWN => Type::Unknown,
            AV::AVMEDIA_TYPE_VIDEO => Type::Video,
            AV::AVMEDIA_TYPE_AUDIO => Type::Audio,
            AV::AVMEDIA_TYPE_DATA => Type::Data,
            AV::AVMEDIA_TYPE_SUBTITLE => Type::Subtitle,
            AV::AVMEDIA_TYPE_ATTACHMENT => Type::Attachment,
            AV::AVMEDIA_TYPE_NB => Type::Unknown,

            #[cfg(feature = "non-exhaustive-enums")]
            _ => unimplemented!(),
        }
    }
}

impl From<Type> for AVMediaType {
    #[inline(always)]
    fn from(value: Type) -> AVMediaType {
        match value {
            Type::Unknown => Self::AVMEDIA_TYPE_UNKNOWN,
            Type::Video => Self::AVMEDIA_TYPE_VIDEO,
            Type::Audio => Self::AVMEDIA_TYPE_AUDIO,
            Type::Data => Self::AVMEDIA_TYPE_DATA,
            Type::Subtitle => Self::AVMEDIA_TYPE_SUBTITLE,
            Type::Attachment => Self::AVMEDIA_TYPE_ATTACHMENT,
        }
    }
}
