use crate::ffi::*;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum FieldOrder {
    Unknown,
    Progressive,
    TT,
    BB,
    TB,
    BT,
}

impl From<AVFieldOrder> for FieldOrder {
    fn from(value: AVFieldOrder) -> Self {
        use AVFieldOrder as AV;

        match value {
            AV::AV_FIELD_UNKNOWN => FieldOrder::Unknown,
            AV::AV_FIELD_PROGRESSIVE => FieldOrder::Progressive,
            AV::AV_FIELD_TT => FieldOrder::TT,
            AV::AV_FIELD_BB => FieldOrder::BB,
            AV::AV_FIELD_TB => FieldOrder::TB,
            AV::AV_FIELD_BT => FieldOrder::BT,

            #[cfg(feature = "non-exhaustive-enums")]
            _ => unimplemented!(),
        }
    }
}

impl From<FieldOrder> for AVFieldOrder {
    fn from(value: FieldOrder) -> AVFieldOrder {
        match value {
            FieldOrder::Unknown => Self::AV_FIELD_UNKNOWN,
            FieldOrder::Progressive => Self::AV_FIELD_PROGRESSIVE,
            FieldOrder::TT => Self::AV_FIELD_TT,
            FieldOrder::BB => Self::AV_FIELD_BB,
            FieldOrder::TB => Self::AV_FIELD_TB,
            FieldOrder::BT => Self::AV_FIELD_BT,
        }
    }
}
