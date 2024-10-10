use std::ffi::CStr;
use std::str::from_utf8_unchecked;

use super::{Audio, Capabilities, Id, Profile, Video};
use crate::ffi::*;
use crate::{media, Error};

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Codec {
    // pub(super) so that other Codec wrappers can use the field
    pub(super) av_codec: &'static AVCodec,
}

// SAFETY: We ensure that the data being pointed to by fields on
//         `AVCodec` are never written to.
unsafe impl Send for Codec {}
unsafe impl Sync for Codec {}

impl Codec {
    /// Create a new [`Codec`] from a pointer to an [`AVCodec`]. Returns `None` if
    /// the pointer is null.
    /// 
    /// # Safety
    /// 
    /// Callers must ensure that `ptr` is either null or:
    /// - Properly aligned for `AVCodec`
    /// - Valid for read accesses
    /// - pointing to initialized memory and
    /// - pointing to a static instance of `AVCodec`, i.e. the memory will live
    ///   and remain unmutated for the entire lifetime of the program.
    /// 
    /// The easiest way to ensure this is using `const *AVCodec`
    /// pointers returned by the FFmpeg API.
    pub unsafe fn from_ptr(ptr: *const AVCodec) -> Option<Self> {
        unsafe { ptr.as_ref().map(|av_codec| Self { av_codec }) }
    }

    /// Returns a raw pointer to the underlying [`AVCodec`] instance.
    pub fn as_ptr(&self) -> *const AVCodec {
        self.av_codec as *const _
    }

    pub fn is_encoder(&self) -> bool {
        unsafe { av_codec_is_encoder(self.as_ptr()) != 0 }
    }

    pub fn is_decoder(&self) -> bool {
        unsafe { av_codec_is_decoder(self.as_ptr()) != 0 }
    }

    pub fn name(&self) -> &str {
        unsafe { from_utf8_unchecked(CStr::from_ptr(self.av_codec.name).to_bytes()) }
    }

    pub fn description(&self) -> &str {
        let long_name = self.av_codec.long_name;
        unsafe {
            if long_name.is_null() {
                ""
            } else {
                from_utf8_unchecked(CStr::from_ptr(long_name).to_bytes())
            }
        }
    }

    pub fn medium(&self) -> media::Type {
        self.av_codec.type_.into()
    }

    pub fn id(&self) -> Id {
        self.av_codec.id.into()
    }

    pub fn is_video(&self) -> bool {
        self.medium() == media::Type::Video
    }

    pub fn video(self) -> Result<Video, Error> {
        if self.medium() == media::Type::Video {
            Ok(Video::new(self))
        } else {
            Err(Error::InvalidData)
        }
    }

    pub fn is_audio(&self) -> bool {
        self.medium() == media::Type::Audio
    }

    pub fn audio(self) -> Result<Audio, Error> {
        if self.medium() == media::Type::Audio {
            Ok(Audio::new(self))
        } else {
            Err(Error::InvalidData)
        }
    }

    pub fn max_lowres(&self) -> i32 {
        self.av_codec.max_lowres.into()
    }

    pub fn capabilities(&self) -> Capabilities {
        Capabilities::from_bits_truncate(self.av_codec.capabilities as u32)
    }

    pub fn profiles(&self) -> Option<ProfileIter> {
        if self.av_codec.profiles.is_null() {
            None
        } else {
            Some(ProfileIter::new(self.id(), self.av_codec.profiles))
        }
    }
}

pub struct ProfileIter {
    id: Id,
    ptr: *const AVProfile,
}

impl ProfileIter {
    pub fn new(id: Id, ptr: *const AVProfile) -> Self {
        ProfileIter { id, ptr }
    }
}

impl Iterator for ProfileIter {
    type Item = Profile;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if (*self.ptr).profile == FF_PROFILE_UNKNOWN {
                return None;
            }

            let profile = Profile::from((self.id, (*self.ptr).profile));
            self.ptr = self.ptr.offset(1);

            Some(profile)
        }
    }
}
