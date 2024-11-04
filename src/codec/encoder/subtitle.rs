use std::ops::{Deref, DerefMut};
use std::ptr;

use crate::ffi::*;
use libc::c_int;

use super::Encoder as Super;
use crate::codec::traits;
use crate::{Dictionary, Error};

pub struct Subtitle(pub Super);

impl Subtitle {}

impl Deref for Subtitle {
    type Target = Super;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Subtitle {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

pub struct Encoder(pub Subtitle);

impl Encoder {
    pub fn encode(&mut self, subtitle: &crate::Subtitle, out: &mut [u8]) -> Result<bool, Error> {
        unsafe {
            match avcodec_encode_subtitle(
                self.0.as_mut_ptr(),
                out.as_mut_ptr(),
                out.len() as c_int,
                subtitle.as_ptr(),
            ) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(true),
            }
        }
    }
}

impl Deref for Encoder {
    type Target = Subtitle;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Encoder {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}
