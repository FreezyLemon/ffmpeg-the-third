use std::ops::{Deref, DerefMut};

use super::{Audio, Check, Conceal, Subtitle, Video};
use crate::codec::Context;
use crate::{Discard, Error, Rational};

pub struct Decoder(pub Context);

impl Decoder {
    pub fn video(self) -> Result<Video, Error> {
        if let Some(codec) = super::find(self.id()) {
            self.open_as(codec).and_then(|o| o.video())
        } else {
            Err(Error::DecoderNotFound)
        }
    }

    pub fn audio(self) -> Result<Audio, Error> {
        if let Some(codec) = super::find(self.id()) {
            self.open_as(codec).and_then(|o| o.audio())
        } else {
            Err(Error::DecoderNotFound)
        }
    }

    pub fn subtitle(self) -> Result<Subtitle, Error> {
        if let Some(codec) = super::find(self.id()) {
            self.open_as(codec).and_then(|o| o.subtitle())
        } else {
            Err(Error::DecoderNotFound)
        }
    }

    pub fn conceal(&mut self, value: Conceal) {
        unsafe {
            (*self.as_mut_ptr()).error_concealment = value.bits();
        }
    }

    pub fn check(&mut self, value: Check) {
        unsafe {
            (*self.as_mut_ptr()).err_recognition = value.bits();
        }
    }

    pub fn skip_loop_filter(&mut self, value: Discard) {
        unsafe {
            (*self.as_mut_ptr()).skip_loop_filter = value.into();
        }
    }

    pub fn skip_idct(&mut self, value: Discard) {
        unsafe {
            (*self.as_mut_ptr()).skip_idct = value.into();
        }
    }

    pub fn skip_frame(&mut self, value: Discard) {
        unsafe {
            (*self.as_mut_ptr()).skip_frame = value.into();
        }
    }

    pub fn time_base(&self) -> Rational {
        unsafe { Rational::from((*self.as_ptr()).time_base) }
    }
}

impl Deref for Decoder {
    type Target = Context;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Decoder {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}
