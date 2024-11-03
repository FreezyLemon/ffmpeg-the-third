use std::marker::PhantomData;
use std::slice;

use super::Packet;
use crate::ffi::*;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Type {
    Palette,
    NewExtraData,
    ParamChange,
    H263MbInfo,
    ReplayGain,
    DisplayMatrix,
    Stereo3d,
    AudioServiceType,
    QualityStats,
    FallbackTrack,
    CBPProperties,
    SkipSamples,
    JpDualMono,
    StringsMetadata,
    SubtitlePosition,
    MatroskaBlockAdditional,
    WebVTTIdentifier,
    WebVTTSettings,
    MetadataUpdate,
    MPEGTSStreamID,
    MasteringDisplayMetadata,
    DataSpherical,
    DataNb,

    ContentLightLevel,
    A53CC,

    EncryptionInitInfo,
    EncryptionInfo,

    AFD,

    #[cfg(feature = "ffmpeg_4_3")]
    PRFT,
    #[cfg(feature = "ffmpeg_4_3")]
    ICC_PROFILE,
    #[cfg(feature = "ffmpeg_4_3")]
    DOVI_CONF,

    #[cfg(feature = "ffmpeg_4_4")]
    S12M_TIMECODE,

    #[cfg(feature = "ffmpeg_5_0")]
    DYNAMIC_HDR10_PLUS,

    #[cfg(feature = "ffmpeg_7_0")]
    IAMF_MIX_GAIN_PARAM,
    #[cfg(feature = "ffmpeg_7_0")]
    IAMF_DEMIXING_INFO_PARAM,
    #[cfg(feature = "ffmpeg_7_0")]
    IAMF_RECON_GAIN_INFO_PARAM,
    #[cfg(feature = "ffmpeg_7_0")]
    AMBIENT_VIEWING_ENVIRONMENT,

    #[cfg(feature = "ffmpeg_7_1")]
    FrameCropping,
    #[cfg(feature = "ffmpeg_7_1")]
    LCEVC,
}

impl From<AVPacketSideDataType> for Type {
    fn from(value: AVPacketSideDataType) -> Self {
        use AVPacketSideDataType as AV;

        match value {
            AV::AV_PKT_DATA_PALETTE => Type::Palette,
            AV::AV_PKT_DATA_NEW_EXTRADATA => Type::NewExtraData,
            AV::AV_PKT_DATA_PARAM_CHANGE => Type::ParamChange,
            AV::AV_PKT_DATA_H263_MB_INFO => Type::H263MbInfo,
            AV::AV_PKT_DATA_REPLAYGAIN => Type::ReplayGain,
            AV::AV_PKT_DATA_DISPLAYMATRIX => Type::DisplayMatrix,
            AV::AV_PKT_DATA_STEREO3D => Type::Stereo3d,
            AV::AV_PKT_DATA_AUDIO_SERVICE_TYPE => Type::AudioServiceType,
            AV::AV_PKT_DATA_QUALITY_STATS => Type::QualityStats,
            AV::AV_PKT_DATA_FALLBACK_TRACK => Type::FallbackTrack,
            AV::AV_PKT_DATA_CPB_PROPERTIES => Type::CBPProperties,
            AV::AV_PKT_DATA_SKIP_SAMPLES => Type::SkipSamples,
            AV::AV_PKT_DATA_JP_DUALMONO => Type::JpDualMono,
            AV::AV_PKT_DATA_STRINGS_METADATA => Type::StringsMetadata,
            AV::AV_PKT_DATA_SUBTITLE_POSITION => Type::SubtitlePosition,
            AV::AV_PKT_DATA_MATROSKA_BLOCKADDITIONAL => Type::MatroskaBlockAdditional,
            AV::AV_PKT_DATA_WEBVTT_IDENTIFIER => Type::WebVTTIdentifier,
            AV::AV_PKT_DATA_WEBVTT_SETTINGS => Type::WebVTTSettings,
            AV::AV_PKT_DATA_METADATA_UPDATE => Type::MetadataUpdate,
            AV::AV_PKT_DATA_MPEGTS_STREAM_ID => Type::MPEGTSStreamID,
            AV::AV_PKT_DATA_MASTERING_DISPLAY_METADATA => Type::MasteringDisplayMetadata,
            AV::AV_PKT_DATA_SPHERICAL => Type::DataSpherical,
            AV::AV_PKT_DATA_NB => Type::DataNb,

            AV::AV_PKT_DATA_CONTENT_LIGHT_LEVEL => Type::ContentLightLevel,
            AV::AV_PKT_DATA_A53_CC => Type::A53CC,

            AV::AV_PKT_DATA_ENCRYPTION_INIT_INFO => Type::EncryptionInitInfo,
            AV::AV_PKT_DATA_ENCRYPTION_INFO => Type::EncryptionInfo,

            AV::AV_PKT_DATA_AFD => Type::AFD,

            #[cfg(feature = "ffmpeg_4_3")]
            AV::AV_PKT_DATA_PRFT => Type::PRFT,
            #[cfg(feature = "ffmpeg_4_3")]
            AV::AV_PKT_DATA_ICC_PROFILE => Type::ICC_PROFILE,
            #[cfg(feature = "ffmpeg_4_3")]
            AV::AV_PKT_DATA_DOVI_CONF => Type::DOVI_CONF,

            #[cfg(feature = "ffmpeg_4_4")]
            AV::AV_PKT_DATA_S12M_TIMECODE => Type::S12M_TIMECODE,

            #[cfg(feature = "ffmpeg_5_0")]
            AV::AV_PKT_DATA_DYNAMIC_HDR10_PLUS => Type::DYNAMIC_HDR10_PLUS,

            #[cfg(feature = "ffmpeg_7_0")]
            AV::AV_PKT_DATA_IAMF_MIX_GAIN_PARAM => Type::IAMF_MIX_GAIN_PARAM,
            #[cfg(feature = "ffmpeg_7_0")]
            AV::AV_PKT_DATA_IAMF_DEMIXING_INFO_PARAM => Type::IAMF_DEMIXING_INFO_PARAM,
            #[cfg(feature = "ffmpeg_7_0")]
            AV::AV_PKT_DATA_IAMF_RECON_GAIN_INFO_PARAM => Type::IAMF_RECON_GAIN_INFO_PARAM,
            #[cfg(feature = "ffmpeg_7_0")]
            AV::AV_PKT_DATA_AMBIENT_VIEWING_ENVIRONMENT => Type::AMBIENT_VIEWING_ENVIRONMENT,

            #[cfg(feature = "ffmpeg_7_1")]
            AV::AV_PKT_DATA_FRAME_CROPPING => Type::FrameCropping,
            #[cfg(feature = "ffmpeg_7_1")]
            AV::AV_PKT_DATA_LCEVC => Type::LCEVC,

            #[cfg(feature = "non-exhaustive-enums")]
            _ => unimplemented!(),
        }
    }
}

impl From<Type> for AVPacketSideDataType {
    fn from(value: Type) -> AVPacketSideDataType {
        match value {
            Type::Palette => Self::AV_PKT_DATA_PALETTE,
            Type::NewExtraData => Self::AV_PKT_DATA_NEW_EXTRADATA,
            Type::ParamChange => Self::AV_PKT_DATA_PARAM_CHANGE,
            Type::H263MbInfo => Self::AV_PKT_DATA_H263_MB_INFO,
            Type::ReplayGain => Self::AV_PKT_DATA_REPLAYGAIN,
            Type::DisplayMatrix => Self::AV_PKT_DATA_DISPLAYMATRIX,
            Type::Stereo3d => Self::AV_PKT_DATA_STEREO3D,
            Type::AudioServiceType => Self::AV_PKT_DATA_AUDIO_SERVICE_TYPE,
            Type::QualityStats => Self::AV_PKT_DATA_QUALITY_STATS,
            Type::FallbackTrack => Self::AV_PKT_DATA_FALLBACK_TRACK,
            Type::CBPProperties => Self::AV_PKT_DATA_CPB_PROPERTIES,
            Type::SkipSamples => Self::AV_PKT_DATA_SKIP_SAMPLES,
            Type::JpDualMono => Self::AV_PKT_DATA_JP_DUALMONO,
            Type::StringsMetadata => Self::AV_PKT_DATA_STRINGS_METADATA,
            Type::SubtitlePosition => Self::AV_PKT_DATA_SUBTITLE_POSITION,
            Type::MatroskaBlockAdditional => Self::AV_PKT_DATA_MATROSKA_BLOCKADDITIONAL,
            Type::WebVTTIdentifier => Self::AV_PKT_DATA_WEBVTT_IDENTIFIER,
            Type::WebVTTSettings => Self::AV_PKT_DATA_WEBVTT_SETTINGS,
            Type::MetadataUpdate => Self::AV_PKT_DATA_METADATA_UPDATE,
            Type::MPEGTSStreamID => Self::AV_PKT_DATA_MPEGTS_STREAM_ID,
            Type::MasteringDisplayMetadata => Self::AV_PKT_DATA_MASTERING_DISPLAY_METADATA,
            Type::DataSpherical => Self::AV_PKT_DATA_SPHERICAL,
            Type::DataNb => Self::AV_PKT_DATA_NB,

            Type::ContentLightLevel => Self::AV_PKT_DATA_CONTENT_LIGHT_LEVEL,
            Type::A53CC => Self::AV_PKT_DATA_A53_CC,

            Type::EncryptionInitInfo => Self::AV_PKT_DATA_ENCRYPTION_INIT_INFO,
            Type::EncryptionInfo => Self::AV_PKT_DATA_ENCRYPTION_INFO,

            Type::AFD => Self::AV_PKT_DATA_AFD,

            #[cfg(feature = "ffmpeg_4_3")]
            Type::PRFT => Self::AV_PKT_DATA_PRFT,
            #[cfg(feature = "ffmpeg_4_3")]
            Type::ICC_PROFILE => Self::AV_PKT_DATA_ICC_PROFILE,
            #[cfg(feature = "ffmpeg_4_3")]
            Type::DOVI_CONF => Self::AV_PKT_DATA_DOVI_CONF,

            #[cfg(feature = "ffmpeg_4_4")]
            Type::S12M_TIMECODE => Self::AV_PKT_DATA_S12M_TIMECODE,

            #[cfg(feature = "ffmpeg_5_0")]
            Type::DYNAMIC_HDR10_PLUS => Self::AV_PKT_DATA_DYNAMIC_HDR10_PLUS,

            #[cfg(feature = "ffmpeg_7_0")]
            Type::IAMF_MIX_GAIN_PARAM => Self::AV_PKT_DATA_IAMF_MIX_GAIN_PARAM,
            #[cfg(feature = "ffmpeg_7_0")]
            Type::IAMF_DEMIXING_INFO_PARAM => Self::AV_PKT_DATA_IAMF_DEMIXING_INFO_PARAM,
            #[cfg(feature = "ffmpeg_7_0")]
            Type::IAMF_RECON_GAIN_INFO_PARAM => Self::AV_PKT_DATA_IAMF_RECON_GAIN_INFO_PARAM,
            #[cfg(feature = "ffmpeg_7_0")]
            Type::AMBIENT_VIEWING_ENVIRONMENT => Self::AV_PKT_DATA_AMBIENT_VIEWING_ENVIRONMENT,

            #[cfg(feature = "ffmpeg_7_1")]
            Type::FrameCropping => Self::AV_PKT_DATA_FRAME_CROPPING,
            #[cfg(feature = "ffmpeg_7_1")]
            Type::LCEVC => Self::AV_PKT_DATA_LCEVC,
        }
    }
}

pub struct SideData<'a> {
    ptr: *mut AVPacketSideData,

    _marker: PhantomData<&'a Packet>,
}

impl<'a> SideData<'a> {
    pub unsafe fn wrap(ptr: *mut AVPacketSideData) -> Self {
        SideData {
            ptr,
            _marker: PhantomData,
        }
    }

    pub unsafe fn as_ptr(&self) -> *const AVPacketSideData {
        self.ptr as *const _
    }
}

impl<'a> SideData<'a> {
    pub fn kind(&self) -> Type {
        unsafe { Type::from((*self.as_ptr()).type_) }
    }

    pub fn data(&self) -> &[u8] {
        unsafe { slice::from_raw_parts((*self.as_ptr()).data, (*self.as_ptr()).size as usize) }
    }
}
