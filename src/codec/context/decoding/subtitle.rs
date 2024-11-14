use crate::ffi::*;
use crate::packet;
use crate::Error;
use libc::c_int;

use super::{State, SubtitleDecoder};

impl<S: State> SubtitleDecoder<S> {
    pub fn decode<P: packet::Ref>(
        &mut self,
        packet: &P,
        out: &mut crate::Subtitle,
    ) -> Result<bool, Error> {
        unsafe {
            let mut got: c_int = 0;

            match avcodec_decode_subtitle2(
                self.as_mut_ptr(),
                out.as_mut_ptr(),
                &mut got,
                packet.as_ptr() as *mut _,
            ) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(got != 0),
            }
        }
    }
}
