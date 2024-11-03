use crate::ffi::AVChannelOrder;

/// Specifies an order for audio channels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelOrder {
    /// No channel order. Only the channel count is specified.
    Unspecified,

    /// Native channel order, i.e. the channels are in the same order in which they
    /// are defined in the [`Channel`][super::Channel] enum. This supports up to 63 channels.
    Native,

    /// The channel order does not correspond to any predefined order and is stored as an
    /// explicit map. This can be used to support layouts with more than 64 channels or with
    /// empty channels at arbitrary positions.
    Custom,

    /// The audio is represented as the decomposition of the sound field into spherical harmonics.
    Ambisonic,
}

impl From<AVChannelOrder> for ChannelOrder {
    fn from(value: AVChannelOrder) -> Self {
        use AVChannelOrder as AV;

        match value {
            AV::AV_CHANNEL_ORDER_UNSPEC => Self::Unspecified,
            AV::AV_CHANNEL_ORDER_NATIVE => Self::Native,
            AV::AV_CHANNEL_ORDER_CUSTOM => Self::Custom,
            AV::AV_CHANNEL_ORDER_AMBISONIC => Self::Ambisonic,
            #[cfg(feature = "ffmpeg_7_0")]
            // Not part of the API, should never be used
            AV::FF_CHANNEL_ORDER_NB => unreachable!(),
            #[cfg(feature = "non-exhaustive-enums")]
            _ => unimplemented!(),
        }
    }
}

impl From<ChannelOrder> for AVChannelOrder {
    fn from(value: ChannelOrder) -> Self {
        use ChannelOrder as O;

        match value {
            O::Unspecified => Self::AV_CHANNEL_ORDER_UNSPEC,
            O::Native => Self::AV_CHANNEL_ORDER_NATIVE,
            O::Custom => Self::AV_CHANNEL_ORDER_CUSTOM,
            O::Ambisonic => Self::AV_CHANNEL_ORDER_AMBISONIC,
        }
    }
}
