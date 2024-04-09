use crate::ffi::AVChannelOrder::*;
use crate::ffi::*;

pub enum ChannelOrder {
    Unspecified,
    Native,
    Custom,
    Ambisonic,
}

impl From<AVChannelOrder> for ChannelOrder {
    fn from(value: AVChannelOrder) -> Self {
        match value {
            AV_CHANNEL_ORDER_UNSPEC => ChannelOrder::Unspecified,
            AV_CHANNEL_ORDER_NATIVE => ChannelOrder::Native,
            AV_CHANNEL_ORDER_CUSTOM => ChannelOrder::Custom,
            AV_CHANNEL_ORDER_AMBISONIC => ChannelOrder::Ambisonic,
        }
    }
}
