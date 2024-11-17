use std::ops::{Deref, DerefMut};
use std::ptr;

use crate::codec::Profile;
use crate::ffi::*;
use crate::{media, packet, Error, Frame, Rational};

pub struct Opened(pub Decoder);

impl Opened {
    pub fn send_packet<P: packet::Ref>(&mut self, packet: &P) -> Result<(), Error> {
        unsafe {
            match avcodec_send_packet(self.as_mut_ptr(), packet.as_ptr()) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(()),
            }
        }
    }

    /// Sends a NULL packet to the decoder to signal end of stream and enter
    /// draining mode.
    pub fn send_eof(&mut self) -> Result<(), Error> {
        unsafe {
            match avcodec_send_packet(self.as_mut_ptr(), ptr::null()) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(()),
            }
        }
    }

    pub fn receive_frame(&mut self, frame: &mut Frame) -> Result<(), Error> {
        unsafe {
            match avcodec_receive_frame(self.as_mut_ptr(), frame.as_mut_ptr()) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(()),
            }
        }
    }

    pub fn delay(&self) -> usize {
        unsafe { (*self.as_ptr()).delay as usize }
    }

    pub fn profile(&self) -> Profile {
        unsafe { Profile::from((self.id(), (*self.as_ptr()).profile)) }
    }

    pub fn frame_rate(&self) -> Option<Rational> {
        unsafe {
            let value = (*self.as_ptr()).framerate;

            if value == (AVRational { num: 0, den: 1 }) {
                None
            } else {
                Some(Rational::from(value))
            }
        }
    }

    pub fn flush(&mut self) {
        unsafe {
            avcodec_flush_buffers(self.as_mut_ptr());
        }
    }
}

impl Drop for Opened {
    fn drop(&mut self) {
        unsafe {
            avcodec_close(self.as_mut_ptr());
        }
    }
}

impl Deref for Opened {
    type Target = Decoder;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Opened {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}
