use super::Encoder;

use crate::AsMutPtr;
use crate::Rational;

use libc::c_int;

impl<T, S> Encoder<T, S> {
    pub fn set_bit_rate(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).bit_rate = value as i64;
        }
    }
    
    pub fn set_max_bit_rate(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).rc_max_rate = value as i64;
        }
    }
    
    pub fn set_tolerance(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).bit_rate_tolerance = value as c_int;
        }
    }
    
    pub fn set_quality(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).global_quality = value as c_int;
        }
    }
    
    pub fn set_compression(&mut self, value: Option<usize>) {
        unsafe {
            if let Some(value) = value {
                (*self.as_mut_ptr()).compression_level = value as c_int;
            } else {
                (*self.as_mut_ptr()).compression_level = -1;
            }
        }
    }
    
    pub fn set_time_base<R: Into<Rational>>(&mut self, value: R) {
        unsafe {
            (*self.as_mut_ptr()).time_base = value.into().into();
        }
    }
    
    pub fn set_frame_rate<R: Into<Rational>>(&mut self, value: Option<R>) {
        unsafe {
            if let Some(value) = value {
                (*self.as_mut_ptr()).framerate = value.into().into();
            } else {
                (*self.as_mut_ptr()).framerate.num = 0;
                (*self.as_mut_ptr()).framerate.den = 1;
            }
        }
    }
}
