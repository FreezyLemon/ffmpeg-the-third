use std::ops::{Deref, DerefMut};
use std::ptr;

use crate::codec::Profile;
use crate::ffi::*;
use crate::{media, packet, Error, Frame, Rational};

pub struct Opened(pub Decoder);

impl Opened {
    pub fn profile(&self) -> Profile {
        unsafe { Profile::from((self.id(), (*self.as_ptr()).profile)) }
    }
}
