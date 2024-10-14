use std::ffi::CStr;

use super::{Audio, Capabilities, Id, Profile, Video};
use crate::ffi::*;
use crate::macros::{impl_field_string, impl_getter_into, impl_ref_wrapper};
use crate::{media, Error};

impl_ref_wrapper!(Codec, AVCodec);

unsafe impl<'c> Send for Codec<'c> {}
unsafe impl<'c> Sync for Codec<'c> {}

impl<'c> Codec<'c> {
    pub fn is_encoder(&self) -> bool {
        unsafe { av_codec_is_encoder(self.as_ptr()) != 0 }
    }

    pub fn is_decoder(&self) -> bool {
        unsafe { av_codec_is_decoder(self.as_ptr()) != 0 }
    }

    impl_field_string!(name, name);
    impl_field_string!(optional description, long_name);
    impl_getter_into!(medium() -> media::Type; type_);
    impl_getter_into!(id() -> Id; id);

    pub fn is_video(&self) -> bool {
        self.medium() == media::Type::Video
    }

    pub fn video(self) -> Result<Video<'c>, Error> {
        unsafe {
            if self.medium() == media::Type::Video {
                Ok(Video::new(self))
            } else {
                Err(Error::InvalidData)
            }
        }
    }

    pub fn is_audio(&self) -> bool {
        self.medium() == media::Type::Audio
    }

    pub fn audio(self) -> Result<Audio<'c>, Error> {
        unsafe {
            if self.medium() == media::Type::Audio {
                Ok(Audio::new(self))
            } else {
                Err(Error::InvalidData)
            }
        }
    }

    impl_getter_into!(max_lowres() -> i32; max_lowres);

    pub fn capabilities(&self) -> Capabilities {
        unsafe { Capabilities::from_bits_truncate((*self.as_ptr()).capabilities as u32) }
    }

    pub fn profiles(&self) -> Option<ProfileIter> {
        unsafe {
            if (*self.as_ptr()).profiles.is_null() {
                None
            } else {
                Some(ProfileIter::new(self.id(), (*self.as_ptr()).profiles))
            }
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
