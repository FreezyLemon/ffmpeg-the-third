use crate::ffi::*;
use crate::utils;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum TransferCharacteristic {
    Reserved0,
    BT709,
    Unspecified,
    Reserved,
    GAMMA22,
    GAMMA28,
    SMPTE170M,
    SMPTE240M,
    Linear,
    Log,
    LogSqrt,
    IEC61966_2_4,
    BT1361_ECG,
    IEC61966_2_1,
    BT2020_10,
    BT2020_12,
    SMPTE2084,
    SMPTE428,
    ARIB_STD_B67,
}

impl TransferCharacteristic {
    pub fn name(&self) -> Option<&'static str> {
        if *self == TransferCharacteristic::Unspecified {
            return None;
        }
        unsafe {
            let ptr = av_color_transfer_name((*self).into());
            utils::optional_str_from_c_ptr(ptr)
        }
    }
}

impl From<AVColorTransferCharacteristic> for TransferCharacteristic {
    fn from(value: AVColorTransferCharacteristic) -> TransferCharacteristic {
        use AVColorTransferCharacteristic as AV;

        match value {
            AV::AVCOL_TRC_RESERVED0 => Self::Reserved0,
            AV::AVCOL_TRC_BT709 => Self::BT709,
            AV::AVCOL_TRC_UNSPECIFIED => Self::Unspecified,
            AV::AVCOL_TRC_RESERVED => Self::Reserved,
            AV::AVCOL_TRC_GAMMA22 => Self::GAMMA22,
            AV::AVCOL_TRC_GAMMA28 => Self::GAMMA28,
            AV::AVCOL_TRC_SMPTE170M => Self::SMPTE170M,
            AV::AVCOL_TRC_SMPTE240M => Self::SMPTE240M,
            AV::AVCOL_TRC_LINEAR => Self::Linear,
            AV::AVCOL_TRC_LOG => Self::Log,
            AV::AVCOL_TRC_LOG_SQRT => Self::LogSqrt,
            AV::AVCOL_TRC_IEC61966_2_4 => Self::IEC61966_2_4,
            AV::AVCOL_TRC_BT1361_ECG => Self::BT1361_ECG,
            AV::AVCOL_TRC_IEC61966_2_1 => Self::IEC61966_2_1,
            AV::AVCOL_TRC_BT2020_10 => Self::BT2020_10,
            AV::AVCOL_TRC_BT2020_12 => Self::BT2020_12,
            AV::AVCOL_TRC_NB => Self::Reserved0,
            AV::AVCOL_TRC_SMPTE2084 => Self::SMPTE2084,
            AV::AVCOL_TRC_SMPTE428 => Self::SMPTE428,
            AV::AVCOL_TRC_ARIB_STD_B67 => Self::ARIB_STD_B67,

            #[cfg(feature = "non-exhaustive-enums")]
            _ => unimplemented!(),
        }
    }
}

impl From<TransferCharacteristic> for AVColorTransferCharacteristic {
    fn from(value: TransferCharacteristic) -> AVColorTransferCharacteristic {
        use TransferCharacteristic as TC;

        match value {
            TC::Reserved0 => Self::AVCOL_TRC_RESERVED0,
            TC::BT709 => Self::AVCOL_TRC_BT709,
            TC::Unspecified => Self::AVCOL_TRC_UNSPECIFIED,
            TC::Reserved => Self::AVCOL_TRC_RESERVED,
            TC::GAMMA22 => Self::AVCOL_TRC_GAMMA22,
            TC::GAMMA28 => Self::AVCOL_TRC_GAMMA28,
            TC::SMPTE170M => Self::AVCOL_TRC_SMPTE170M,
            TC::SMPTE240M => Self::AVCOL_TRC_SMPTE240M,
            TC::Linear => Self::AVCOL_TRC_LINEAR,
            TC::Log => Self::AVCOL_TRC_LOG,
            TC::LogSqrt => Self::AVCOL_TRC_LOG_SQRT,
            TC::IEC61966_2_4 => Self::AVCOL_TRC_IEC61966_2_4,
            TC::BT1361_ECG => Self::AVCOL_TRC_BT1361_ECG,
            TC::IEC61966_2_1 => Self::AVCOL_TRC_IEC61966_2_1,
            TC::BT2020_10 => Self::AVCOL_TRC_BT2020_10,
            TC::BT2020_12 => Self::AVCOL_TRC_BT2020_12,
            TC::SMPTE2084 => Self::AVCOL_TRC_SMPTE2084,
            TC::SMPTE428 => Self::AVCOL_TRC_SMPTE428,
            TC::ARIB_STD_B67 => Self::AVCOL_TRC_ARIB_STD_B67,
        }
    }
}
