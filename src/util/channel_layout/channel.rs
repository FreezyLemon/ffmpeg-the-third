use crate::ffi::*;

use std::ffi::CString;

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Channel {
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

    #[cfg(feature = "ffmpeg_7_1")]
    SideSurroundLeft,
    #[cfg(feature = "ffmpeg_7_1")]
    SideSurroundRight,
    #[cfg(feature = "ffmpeg_7_1")]
    TopSurroundLeft,
    #[cfg(feature = "ffmpeg_7_1")]
    TopSurroundRight,

    /// Channel is empty and can be safely skipped.
    Unused,

    /// Channel contains data, but its position is unknown.
    Unknown,

    /// Defines the start of channel IDs when using Ambisonic.
    AmbisonicBase,
    /// Defines the end of channel IDs when using Ambisonic.
    AmbisonicEnd,
}

impl Channel {
    /// Get an abbreviated, human-readable string describing this channel.
    pub fn name(self) -> String {
        let mut buf = vec![0u8; 32];

        unsafe {
            let ret_val = av_channel_name(buf.as_mut_ptr() as _, buf.len(), AVChannel::from(self));

            match usize::try_from(ret_val) {
                Ok(out_len) if out_len > 0 => {
                    #[cfg(feature = "ffmpeg_6_1")]
                    // 6.1 changed out_len to include the NUL byte, which we don't want
                    let out_len = out_len - 1;

                    buf.truncate(out_len);
                    String::from_utf8_unchecked(buf)
                }
                // `av_channel_name` returned an error, or 0 bytes written.
                _ => String::new(),
            }
        }
    }

    /// Get a human-readable string describing this channel.
    pub fn description(self) -> String {
        let mut buf = vec![0u8; 256];

        unsafe {
            let ret_val =
                av_channel_description(buf.as_mut_ptr() as _, buf.len(), AVChannel::from(self));

            match usize::try_from(ret_val) {
                Ok(out_len) if out_len > 0 => {
                    #[cfg(feature = "ffmpeg_6_1")]
                    // 6.1 changed out_len to include the NUL byte, which we don't want
                    let out_len = out_len - 1;

                    buf.truncate(out_len);
                    String::from_utf8_unchecked(buf)
                }
                // `av_channel_description` returned an error, or 0 bytes written.
                _ => String::new(),
            }
        }
    }

    /// This is the inverse function of [`name`][Channel::name].
    pub fn from_string<S: AsRef<str>>(name: S) -> Self {
        let cstr = CString::new(name.as_ref()).expect("no nul byte in name");
        Self::from(unsafe { av_channel_from_string(cstr.as_ptr()) })
    }
}

impl From<AVChannel> for Channel {
    fn from(value: AVChannel) -> Self {
        use crate::ffi::AVChannel as AV;

        match value {
            AV::AV_CHAN_NONE => Self::None,
            AV::AV_CHAN_FRONT_LEFT => Self::FrontLeft,
            AV::AV_CHAN_FRONT_RIGHT => Self::FrontRight,
            AV::AV_CHAN_FRONT_CENTER => Self::FrontCenter,
            AV::AV_CHAN_LOW_FREQUENCY => Self::LowFrequency,
            AV::AV_CHAN_BACK_LEFT => Self::BackLeft,
            AV::AV_CHAN_BACK_RIGHT => Self::BackRight,
            AV::AV_CHAN_FRONT_LEFT_OF_CENTER => Self::FrontLeftOfCenter,
            AV::AV_CHAN_FRONT_RIGHT_OF_CENTER => Self::FrontRightOfCenter,
            AV::AV_CHAN_BACK_CENTER => Self::BackCenter,
            AV::AV_CHAN_SIDE_LEFT => Self::SideLeft,
            AV::AV_CHAN_SIDE_RIGHT => Self::SideRight,
            AV::AV_CHAN_TOP_CENTER => Self::TopCenter,
            AV::AV_CHAN_TOP_FRONT_LEFT => Self::TopFrontLeft,
            AV::AV_CHAN_TOP_FRONT_CENTER => Self::TopFrontCenter,
            AV::AV_CHAN_TOP_FRONT_RIGHT => Self::TopFrontRight,
            AV::AV_CHAN_TOP_BACK_LEFT => Self::TopBackLeft,
            AV::AV_CHAN_TOP_BACK_CENTER => Self::TopBackCenter,
            AV::AV_CHAN_TOP_BACK_RIGHT => Self::TopBackRight,
            AV::AV_CHAN_STEREO_LEFT => Self::StereoLeft,
            AV::AV_CHAN_STEREO_RIGHT => Self::StereoRight,
            AV::AV_CHAN_WIDE_LEFT => Self::WideLeft,
            AV::AV_CHAN_WIDE_RIGHT => Self::WideRight,
            AV::AV_CHAN_SURROUND_DIRECT_LEFT => Self::SurroundDirectLeft,
            AV::AV_CHAN_SURROUND_DIRECT_RIGHT => Self::SurroundDirectRight,
            AV::AV_CHAN_LOW_FREQUENCY_2 => Self::LowFrequency2,
            AV::AV_CHAN_TOP_SIDE_LEFT => Self::TopSideLeft,
            AV::AV_CHAN_TOP_SIDE_RIGHT => Self::TopSideRight,
            AV::AV_CHAN_BOTTOM_FRONT_CENTER => Self::BottomFrontCenter,
            AV::AV_CHAN_BOTTOM_FRONT_LEFT => Self::BottomFrontLeft,
            AV::AV_CHAN_BOTTOM_FRONT_RIGHT => Self::BottomFrontRight,

            #[cfg(feature = "ffmpeg_7_1")]
            AV::AV_CHAN_SIDE_SURROUND_LEFT => Self::SideSurroundLeft,
            #[cfg(feature = "ffmpeg_7_1")]
            AV::AV_CHAN_SIDE_SURROUND_RIGHT => Self::SideSurroundRight,
            #[cfg(feature = "ffmpeg_7_1")]
            AV::AV_CHAN_TOP_SURROUND_LEFT => Self::TopSurroundLeft,
            #[cfg(feature = "ffmpeg_7_1")]
            AV::AV_CHAN_TOP_SURROUND_RIGHT => Self::TopSurroundRight,

            AV::AV_CHAN_UNUSED => Self::Unused,
            AV::AV_CHAN_UNKNOWN => Self::Unknown,
            AV::AV_CHAN_AMBISONIC_BASE => Self::AmbisonicBase,
            AV::AV_CHAN_AMBISONIC_END => Self::AmbisonicEnd,

            #[cfg(feature = "non-exhaustive-enums")]
            _ => unimplemented!(),
        }
    }
}

impl From<Channel> for AVChannel {
    fn from(value: Channel) -> Self {
        use Channel as C;

        match value {
            C::None => Self::AV_CHAN_NONE,
            C::FrontLeft => Self::AV_CHAN_FRONT_LEFT,
            C::FrontRight => Self::AV_CHAN_FRONT_RIGHT,
            C::FrontCenter => Self::AV_CHAN_FRONT_CENTER,
            C::LowFrequency => Self::AV_CHAN_LOW_FREQUENCY,
            C::BackLeft => Self::AV_CHAN_BACK_LEFT,
            C::BackRight => Self::AV_CHAN_BACK_RIGHT,
            C::FrontLeftOfCenter => Self::AV_CHAN_FRONT_LEFT_OF_CENTER,
            C::FrontRightOfCenter => Self::AV_CHAN_FRONT_RIGHT_OF_CENTER,
            C::BackCenter => Self::AV_CHAN_BACK_CENTER,
            C::SideLeft => Self::AV_CHAN_SIDE_LEFT,
            C::SideRight => Self::AV_CHAN_SIDE_RIGHT,
            C::TopCenter => Self::AV_CHAN_TOP_CENTER,
            C::TopFrontLeft => Self::AV_CHAN_TOP_FRONT_LEFT,
            C::TopFrontCenter => Self::AV_CHAN_TOP_FRONT_CENTER,
            C::TopFrontRight => Self::AV_CHAN_TOP_FRONT_RIGHT,
            C::TopBackLeft => Self::AV_CHAN_TOP_BACK_LEFT,
            C::TopBackCenter => Self::AV_CHAN_TOP_BACK_CENTER,
            C::TopBackRight => Self::AV_CHAN_TOP_BACK_RIGHT,
            C::StereoLeft => Self::AV_CHAN_STEREO_LEFT,
            C::StereoRight => Self::AV_CHAN_STEREO_RIGHT,
            C::WideLeft => Self::AV_CHAN_WIDE_LEFT,
            C::WideRight => Self::AV_CHAN_WIDE_RIGHT,
            C::SurroundDirectLeft => Self::AV_CHAN_SURROUND_DIRECT_LEFT,
            C::SurroundDirectRight => Self::AV_CHAN_SURROUND_DIRECT_RIGHT,
            C::LowFrequency2 => Self::AV_CHAN_LOW_FREQUENCY_2,
            C::TopSideLeft => Self::AV_CHAN_TOP_SIDE_LEFT,
            C::TopSideRight => Self::AV_CHAN_TOP_SIDE_RIGHT,
            C::BottomFrontCenter => Self::AV_CHAN_BOTTOM_FRONT_CENTER,
            C::BottomFrontLeft => Self::AV_CHAN_BOTTOM_FRONT_LEFT,
            C::BottomFrontRight => Self::AV_CHAN_BOTTOM_FRONT_RIGHT,

            #[cfg(feature = "ffmpeg_7_1")]
            C::SideSurroundLeft => Self::AV_CHAN_SIDE_SURROUND_LEFT,
            #[cfg(feature = "ffmpeg_7_1")]
            C::SideSurroundRight => Self::AV_CHAN_SIDE_SURROUND_RIGHT,
            #[cfg(feature = "ffmpeg_7_1")]
            C::TopSurroundLeft => Self::AV_CHAN_TOP_SURROUND_LEFT,
            #[cfg(feature = "ffmpeg_7_1")]
            C::TopSurroundRight => Self::AV_CHAN_TOP_SURROUND_RIGHT,

            C::Unused => Self::AV_CHAN_UNUSED,
            C::Unknown => Self::AV_CHAN_UNKNOWN,
            C::AmbisonicBase => Self::AV_CHAN_AMBISONIC_BASE,
            C::AmbisonicEnd => Self::AV_CHAN_AMBISONIC_END,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // just test everything
    const TEST_VALUES: &[Channel] = &[
        Channel::None,
        Channel::FrontLeft,
        Channel::FrontRight,
        Channel::FrontCenter,
        Channel::LowFrequency,
        Channel::BackLeft,
        Channel::BackRight,
        Channel::FrontLeftOfCenter,
        Channel::FrontRightOfCenter,
        Channel::BackCenter,
        Channel::SideLeft,
        Channel::SideRight,
        Channel::TopCenter,
        Channel::TopFrontLeft,
        Channel::TopFrontCenter,
        Channel::TopFrontRight,
        Channel::TopBackLeft,
        Channel::TopBackCenter,
        Channel::TopBackRight,
        Channel::StereoLeft,
        Channel::StereoRight,
        Channel::WideLeft,
        Channel::WideRight,
        Channel::SurroundDirectLeft,
        Channel::SurroundDirectRight,
        Channel::LowFrequency2,
        Channel::TopSideLeft,
        Channel::TopSideRight,
        Channel::BottomFrontCenter,
        Channel::BottomFrontLeft,
        Channel::BottomFrontRight,
        Channel::Unused,
        Channel::Unknown,
        Channel::AmbisonicBase,
        Channel::AmbisonicEnd,
    ];

    #[test]
    fn name() {
        for ch in TEST_VALUES {
            let name = ch.name();
            assert!(!name.is_empty());
            println!("{name}");
        }
    }

    #[test]
    fn description() {
        for ch in TEST_VALUES {
            let desc = ch.description();
            assert!(!desc.is_empty());
            println!("{desc}");
        }
    }

    #[test]
    fn from_string() {
        for ch in TEST_VALUES {
            let name = ch.name();
            let found = Channel::from_string(name);
            assert_eq!(found, *ch);
        }
    }
}
