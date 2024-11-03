use std::marker::PhantomData;
use std::slice;

use super::Frame;
use crate::ffi::*;
use crate::utils;
use crate::DictionaryRef;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Type {
    PanScan,
    A53CC,
    Stereo3D,
    MatrixEncoding,
    DownMixInfo,
    ReplayGain,
    DisplayMatrix,
    AFD,
    MotionVectors,
    SkipSamples,
    AudioServiceType,
    MasteringDisplayMetadata,
    GOPTimecode,
    Spherical,

    ContentLightLevel,
    IccProfile,

    #[cfg(not(feature = "ffmpeg_5_0"))]
    QPTableProperties,
    #[cfg(not(feature = "ffmpeg_5_0"))]
    QPTableData,

    S12M_TIMECODE,

    DYNAMIC_HDR_PLUS,
    REGIONS_OF_INTEREST,

    #[cfg(feature = "ffmpeg_4_3")]
    VIDEO_ENC_PARAMS,

    #[cfg(feature = "ffmpeg_4_4")]
    SEI_UNREGISTERED,
    #[cfg(feature = "ffmpeg_4_4")]
    FILM_GRAIN_PARAMS,

    #[cfg(feature = "ffmpeg_5_0")]
    DETECTION_BBOXES,
    #[cfg(feature = "ffmpeg_5_0")]
    DOVI_RPU_BUFFER,
    #[cfg(feature = "ffmpeg_5_0")]
    DOVI_METADATA,

    #[cfg(feature = "ffmpeg_5_1")]
    DYNAMIC_HDR_VIVID,

    #[cfg(feature = "ffmpeg_6_0")]
    AMBIENT_VIEWING_ENVIRONMENT,

    #[cfg(feature = "ffmpeg_6_1")]
    VIDEO_HINT,

    #[cfg(feature = "ffmpeg_7_1")]
    LCEVC,
    #[cfg(feature = "ffmpeg_7_1")]
    ViewId,
}

impl Type {
    #[inline]
    pub fn name(&self) -> &'static str {
        unsafe { utils::str_from_c_ptr(av_frame_side_data_name((*self).into())) }
    }
}

impl From<AVFrameSideDataType> for Type {
    #[inline(always)]
    fn from(value: AVFrameSideDataType) -> Self {
        use AVFrameSideDataType as AV;

        match value {
            AV::AV_FRAME_DATA_PANSCAN => Type::PanScan,
            AV::AV_FRAME_DATA_A53_CC => Type::A53CC,
            AV::AV_FRAME_DATA_STEREO3D => Type::Stereo3D,
            AV::AV_FRAME_DATA_MATRIXENCODING => Type::MatrixEncoding,
            AV::AV_FRAME_DATA_DOWNMIX_INFO => Type::DownMixInfo,
            AV::AV_FRAME_DATA_REPLAYGAIN => Type::ReplayGain,
            AV::AV_FRAME_DATA_DISPLAYMATRIX => Type::DisplayMatrix,
            AV::AV_FRAME_DATA_AFD => Type::AFD,
            AV::AV_FRAME_DATA_MOTION_VECTORS => Type::MotionVectors,
            AV::AV_FRAME_DATA_SKIP_SAMPLES => Type::SkipSamples,
            AV::AV_FRAME_DATA_AUDIO_SERVICE_TYPE => Type::AudioServiceType,
            AV::AV_FRAME_DATA_MASTERING_DISPLAY_METADATA => Type::MasteringDisplayMetadata,
            AV::AV_FRAME_DATA_GOP_TIMECODE => Type::GOPTimecode,
            AV::AV_FRAME_DATA_SPHERICAL => Type::Spherical,

            AV::AV_FRAME_DATA_CONTENT_LIGHT_LEVEL => Type::ContentLightLevel,
            AV::AV_FRAME_DATA_ICC_PROFILE => Type::IccProfile,

            #[cfg(not(feature = "ffmpeg_5_0"))]
            AV::AV_FRAME_DATA_QP_TABLE_PROPERTIES => Type::QPTableProperties,
            #[cfg(not(feature = "ffmpeg_5_0"))]
            AV::AV_FRAME_DATA_QP_TABLE_DATA => Type::QPTableData,
            AV::AV_FRAME_DATA_S12M_TIMECODE => Type::S12M_TIMECODE,

            AV::AV_FRAME_DATA_DYNAMIC_HDR_PLUS => Type::DYNAMIC_HDR_PLUS,
            AV::AV_FRAME_DATA_REGIONS_OF_INTEREST => Type::REGIONS_OF_INTEREST,

            #[cfg(feature = "ffmpeg_4_3")]
            AV::AV_FRAME_DATA_VIDEO_ENC_PARAMS => Type::VIDEO_ENC_PARAMS,

            #[cfg(feature = "ffmpeg_4_4")]
            AV::AV_FRAME_DATA_SEI_UNREGISTERED => Type::SEI_UNREGISTERED,
            #[cfg(feature = "ffmpeg_4_4")]
            AV::AV_FRAME_DATA_FILM_GRAIN_PARAMS => Type::FILM_GRAIN_PARAMS,

            #[cfg(feature = "ffmpeg_5_0")]
            AV::AV_FRAME_DATA_DETECTION_BBOXES => Type::DETECTION_BBOXES,
            #[cfg(feature = "ffmpeg_5_0")]
            AV::AV_FRAME_DATA_DOVI_RPU_BUFFER => Type::DOVI_RPU_BUFFER,
            #[cfg(feature = "ffmpeg_5_0")]
            AV::AV_FRAME_DATA_DOVI_METADATA => Type::DOVI_METADATA,

            #[cfg(feature = "ffmpeg_5_1")]
            AV::AV_FRAME_DATA_DYNAMIC_HDR_VIVID => Type::DYNAMIC_HDR_VIVID,

            #[cfg(feature = "ffmpeg_6_0")]
            AV::AV_FRAME_DATA_AMBIENT_VIEWING_ENVIRONMENT => Type::AMBIENT_VIEWING_ENVIRONMENT,

            #[cfg(feature = "ffmpeg_6_1")]
            AV::AV_FRAME_DATA_VIDEO_HINT => Type::VIDEO_HINT,

            #[cfg(feature = "ffmpeg_7_1")]
            AV::AV_FRAME_DATA_LCEVC => Type::LCEVC,
            #[cfg(feature = "ffmpeg_7_1")]
            AV::AV_FRAME_DATA_VIEW_ID => Type::ViewId,

            #[cfg(feature = "non-exhaustive-enums")]
            _ => unimplemented!(),
        }
    }
}

impl From<Type> for AVFrameSideDataType {
    #[inline(always)]
    fn from(value: Type) -> AVFrameSideDataType {
        match value {
            Type::PanScan => Self::AV_FRAME_DATA_PANSCAN,
            Type::A53CC => Self::AV_FRAME_DATA_A53_CC,
            Type::Stereo3D => Self::AV_FRAME_DATA_STEREO3D,
            Type::MatrixEncoding => Self::AV_FRAME_DATA_MATRIXENCODING,
            Type::DownMixInfo => Self::AV_FRAME_DATA_DOWNMIX_INFO,
            Type::ReplayGain => Self::AV_FRAME_DATA_REPLAYGAIN,
            Type::DisplayMatrix => Self::AV_FRAME_DATA_DISPLAYMATRIX,
            Type::AFD => Self::AV_FRAME_DATA_AFD,
            Type::MotionVectors => Self::AV_FRAME_DATA_MOTION_VECTORS,
            Type::SkipSamples => Self::AV_FRAME_DATA_SKIP_SAMPLES,
            Type::AudioServiceType => Self::AV_FRAME_DATA_AUDIO_SERVICE_TYPE,
            Type::MasteringDisplayMetadata => Self::AV_FRAME_DATA_MASTERING_DISPLAY_METADATA,
            Type::GOPTimecode => Self::AV_FRAME_DATA_GOP_TIMECODE,
            Type::Spherical => Self::AV_FRAME_DATA_SPHERICAL,

            Type::ContentLightLevel => Self::AV_FRAME_DATA_CONTENT_LIGHT_LEVEL,
            Type::IccProfile => Self::AV_FRAME_DATA_ICC_PROFILE,

            #[cfg(not(feature = "ffmpeg_5_0"))]
            Type::QPTableProperties => Self::AV_FRAME_DATA_QP_TABLE_PROPERTIES,
            #[cfg(not(feature = "ffmpeg_5_0"))]
            Type::QPTableData => Self::AV_FRAME_DATA_QP_TABLE_DATA,
            Type::S12M_TIMECODE => Self::AV_FRAME_DATA_S12M_TIMECODE,

            Type::DYNAMIC_HDR_PLUS => Self::AV_FRAME_DATA_DYNAMIC_HDR_PLUS,
            Type::REGIONS_OF_INTEREST => Self::AV_FRAME_DATA_REGIONS_OF_INTEREST,

            #[cfg(feature = "ffmpeg_4_3")]
            Type::VIDEO_ENC_PARAMS => Self::AV_FRAME_DATA_VIDEO_ENC_PARAMS,

            #[cfg(feature = "ffmpeg_4_4")]
            Type::SEI_UNREGISTERED => Self::AV_FRAME_DATA_SEI_UNREGISTERED,
            #[cfg(feature = "ffmpeg_4_4")]
            Type::FILM_GRAIN_PARAMS => Self::AV_FRAME_DATA_FILM_GRAIN_PARAMS,

            #[cfg(feature = "ffmpeg_5_0")]
            Type::DETECTION_BBOXES => Self::AV_FRAME_DATA_DETECTION_BBOXES,
            #[cfg(feature = "ffmpeg_5_0")]
            Type::DOVI_RPU_BUFFER => Self::AV_FRAME_DATA_DOVI_RPU_BUFFER,
            #[cfg(feature = "ffmpeg_5_0")]
            Type::DOVI_METADATA => Self::AV_FRAME_DATA_DOVI_METADATA,

            #[cfg(feature = "ffmpeg_5_1")]
            Type::DYNAMIC_HDR_VIVID => Self::AV_FRAME_DATA_DYNAMIC_HDR_VIVID,

            #[cfg(feature = "ffmpeg_6_0")]
            Type::AMBIENT_VIEWING_ENVIRONMENT => Self::AV_FRAME_DATA_AMBIENT_VIEWING_ENVIRONMENT,

            #[cfg(feature = "ffmpeg_6_1")]
            Type::VIDEO_HINT => Self::AV_FRAME_DATA_VIDEO_HINT,

            #[cfg(feature = "ffmpeg_7_1")]
            Type::LCEVC => Self::AV_FRAME_DATA_LCEVC,
            #[cfg(feature = "ffmpeg_7_1")]
            Type::ViewId => Self::AV_FRAME_DATA_VIEW_ID,
        }
    }
}

pub struct SideData<'a> {
    ptr: *mut AVFrameSideData,

    _marker: PhantomData<&'a Frame>,
}

impl<'a> SideData<'a> {
    #[inline(always)]
    pub unsafe fn wrap(ptr: *mut AVFrameSideData) -> Self {
        SideData {
            ptr,
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    pub unsafe fn as_ptr(&self) -> *const AVFrameSideData {
        self.ptr as *const _
    }

    #[inline(always)]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVFrameSideData {
        self.ptr
    }
}

impl<'a> SideData<'a> {
    #[inline]
    pub fn kind(&self) -> Type {
        unsafe { Type::from((*self.as_ptr()).type_) }
    }

    #[inline]
    pub fn data(&self) -> &[u8] {
        unsafe { slice::from_raw_parts((*self.as_ptr()).data, (*self.as_ptr()).size as usize) }
    }

    #[inline]
    pub fn metadata(&self) -> DictionaryRef {
        unsafe { DictionaryRef::wrap((*self.as_ptr()).metadata) }
    }
}
