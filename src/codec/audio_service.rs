use crate::ffi::*;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum AudioService {
    Main,
    Effects,
    VisuallyImpaired,
    HearingImpaired,
    Dialogue,
    Commentary,
    Emergency,
    VoiceOver,
    Karaoke,
}

impl From<AVAudioServiceType> for AudioService {
    fn from(value: AVAudioServiceType) -> Self {
        use AVAudioServiceType as AV;

        match value {
            AV::AV_AUDIO_SERVICE_TYPE_MAIN => AudioService::Main,
            AV::AV_AUDIO_SERVICE_TYPE_EFFECTS => AudioService::Effects,
            AV::AV_AUDIO_SERVICE_TYPE_VISUALLY_IMPAIRED => AudioService::VisuallyImpaired,
            AV::AV_AUDIO_SERVICE_TYPE_HEARING_IMPAIRED => AudioService::HearingImpaired,
            AV::AV_AUDIO_SERVICE_TYPE_DIALOGUE => AudioService::Dialogue,
            AV::AV_AUDIO_SERVICE_TYPE_COMMENTARY => AudioService::Commentary,
            AV::AV_AUDIO_SERVICE_TYPE_EMERGENCY => AudioService::Emergency,
            AV::AV_AUDIO_SERVICE_TYPE_VOICE_OVER => AudioService::VoiceOver,
            AV::AV_AUDIO_SERVICE_TYPE_KARAOKE => AudioService::Karaoke,
            AV::AV_AUDIO_SERVICE_TYPE_NB => AudioService::Main,

            #[cfg(feature = "non-exhaustive-enums")]
            _ => unimplemented!(),
        }
    }
}

impl From<AudioService> for AVAudioServiceType {
    fn from(value: AudioService) -> AVAudioServiceType {
        match value {
            AudioService::Main => Self::AV_AUDIO_SERVICE_TYPE_MAIN,
            AudioService::Effects => Self::AV_AUDIO_SERVICE_TYPE_EFFECTS,
            AudioService::VisuallyImpaired => Self::AV_AUDIO_SERVICE_TYPE_VISUALLY_IMPAIRED,
            AudioService::HearingImpaired => Self::AV_AUDIO_SERVICE_TYPE_HEARING_IMPAIRED,
            AudioService::Dialogue => Self::AV_AUDIO_SERVICE_TYPE_DIALOGUE,
            AudioService::Commentary => Self::AV_AUDIO_SERVICE_TYPE_COMMENTARY,
            AudioService::Emergency => Self::AV_AUDIO_SERVICE_TYPE_EMERGENCY,
            AudioService::VoiceOver => Self::AV_AUDIO_SERVICE_TYPE_VOICE_OVER,
            AudioService::Karaoke => Self::AV_AUDIO_SERVICE_TYPE_KARAOKE,
        }
    }
}
