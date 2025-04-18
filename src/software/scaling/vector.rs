use std::marker::PhantomData;
use std::slice;

use crate::ffi::*;
use libc::{c_double, c_int};

pub struct Vector<'a> {
    ptr: *mut SwsVector,

    _own: bool,
    _marker: PhantomData<&'a ()>,
}

impl<'a> Vector<'a> {
    pub unsafe fn wrap(ptr: *mut SwsVector) -> Self {
        Vector {
            ptr,
            _own: false,
            _marker: PhantomData,
        }
    }

    pub unsafe fn as_ptr(&self) -> *const SwsVector {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut SwsVector {
        self.ptr
    }
}

impl<'a> Vector<'a> {
    pub fn new(length: usize) -> Self {
        unsafe {
            Vector {
                ptr: sws_allocVec(length as c_int),
                _own: true,
                _marker: PhantomData,
            }
        }
    }

    pub fn gaussian(variance: f64, quality: f64) -> Self {
        unsafe {
            Vector {
                ptr: sws_getGaussianVec(variance as c_double, quality as c_double),
                _own: true,
                _marker: PhantomData,
            }
        }
    }

    pub fn scale(&mut self, scalar: f64) {
        unsafe {
            sws_scaleVec(self.as_mut_ptr(), scalar as c_double);
        }
    }

    pub fn normalize(&mut self, height: f64) {
        unsafe {
            sws_normalizeVec(self.as_mut_ptr(), height as c_double);
        }
    }

    pub fn coefficients(&self) -> &[f64] {
        unsafe { slice::from_raw_parts((*self.as_ptr()).coeff, (*self.as_ptr()).length as usize) }
    }

    pub fn coefficients_mut(&self) -> &[f64] {
        unsafe {
            slice::from_raw_parts_mut((*self.as_ptr()).coeff, (*self.as_ptr()).length as usize)
        }
    }
}

impl<'a> Drop for Vector<'a> {
    fn drop(&mut self) {
        unsafe {
            if self._own {
                sws_freeVec(self.as_mut_ptr());
            }
        }
    }
}
