use crate::ffi::*;
use libc::c_int;

use crate::Error;
use super::{State, SubtitleEncoder};

impl<S: State> SubtitleEncoder<S> {
    pub fn encode(&mut self, subtitle: &crate::Subtitle, out: &mut [u8]) -> Result<bool, Error> {
        unsafe {
            match avcodec_encode_subtitle(
                self.as_mut_ptr(),
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
