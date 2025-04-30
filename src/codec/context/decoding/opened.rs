use super::{Decoder, Opened};

use crate::ffi::*;
use crate::AsMutPtr;
use crate::{packet, Error, Frame};

use std::ptr;

impl<T> Decoder<T, Opened> {
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

    pub fn flush(&mut self) {
        unsafe {
            avcodec_flush_buffers(self.as_mut_ptr());
        }
    }
}
