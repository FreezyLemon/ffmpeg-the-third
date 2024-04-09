use crate::ffi::*;
use crate::ffi::AVChannel::*;
use Channel::*;

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
enum Channel {
    None,
    FrontLeft,
    FrontRight,
    FrontCenter,
    LowFrequency,
    BackLeft,
    BackRight,
    FrontLeftOfCenter,
    FrontRightOfCenter,
    BackCenter,
    SideLeft,
    SideRight,
    TopCenter,
    TopFrontLeft,
    TopFrontCenter,
    TopFrontRight,
    TopBackLeft,
    TopBackCenter,
    TopBackRight,
    StereoLeft,
    StereoRight,
    WideLeft,
    WideRight,
    SurroundDirectLeft,
    SurroundDirectRight,
    LowFrequency2,
    TopSideLeft,
    TopSideRight,
    BottomFrontCenter,
    BottomFrontLeft,
    BottomFrontRight,

    /// Channel is empty and can be safely skipped.
    Unused,

    /// Channel contains data, but its position is unknown.
    Unknown,

    /// Defines the start of channel IDs when using Ambisonic.
    AmbisonicBase,
    /// Defines the end of channel IDs when using Ambisonic.
    AmbisonicEnd,
}

impl From<AVChannel> for Channel {
    fn from(value: AVChannel) -> Self {
        match value {
            AV_CHAN_NONE => None,
            AV_CHAN_FRONT_LEFT => FrontLeft,
            AV_CHAN_FRONT_RIGHT => FrontRight,
            AV_CHAN_FRONT_CENTER => FrontCenter,
            AV_CHAN_LOW_FREQUENCY => LowFrequency,
            AV_CHAN_BACK_LEFT => BackLeft,
            AV_CHAN_BACK_RIGHT => BackRight,
            AV_CHAN_FRONT_LEFT_OF_CENTER => FrontLeftOfCenter,
            AV_CHAN_FRONT_RIGHT_OF_CENTER => FrontRightOfCenter,
            AV_CHAN_BACK_CENTER => BackCenter,
            AV_CHAN_SIDE_LEFT => SideLeft,
            AV_CHAN_SIDE_RIGHT => SideRight,
            AV_CHAN_TOP_CENTER => TopCenter,
            AV_CHAN_TOP_FRONT_LEFT => TopFrontLeft,
            AV_CHAN_TOP_FRONT_CENTER => TopFrontCenter,
            AV_CHAN_TOP_FRONT_RIGHT => TopFrontRight,
            AV_CHAN_TOP_BACK_LEFT => TopBackLeft,
            AV_CHAN_TOP_BACK_CENTER => TopBackCenter,
            AV_CHAN_TOP_BACK_RIGHT => TopBackRight,
            AV_CHAN_STEREO_LEFT => StereoLeft,
            AV_CHAN_STEREO_RIGHT => StereoRight,
            AV_CHAN_WIDE_LEFT => WideLeft,
            AV_CHAN_WIDE_RIGHT => WideRight,
            AV_CHAN_SURROUND_DIRECT_LEFT => SurroundDirectLeft,
            AV_CHAN_SURROUND_DIRECT_RIGHT => SurroundDirectRight,
            AV_CHAN_LOW_FREQUENCY_2 => LowFrequency2,
            AV_CHAN_TOP_SIDE_LEFT => TopSideLeft,
            AV_CHAN_TOP_SIDE_RIGHT => TopSideRight,
            AV_CHAN_BOTTOM_FRONT_CENTER => BottomFrontCenter,
            AV_CHAN_BOTTOM_FRONT_LEFT => BottomFrontLeft,
            AV_CHAN_BOTTOM_FRONT_RIGHT => BottomFrontRight,
            AV_CHAN_UNUSED => Unused,
            AV_CHAN_UNKNOWN => Unknown,
            AV_CHAN_AMBISONIC_BASE => AmbisonicBase,
            AV_CHAN_AMBISONIC_END => AmbisonicEnd,

            #[cfg(feature = "non-exhaustive-enums")]
            _ => unimplemented!(),
        }
    }
}
