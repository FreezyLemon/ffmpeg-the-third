use std::marker::PhantomData;

use super::{Flags, Pad};
use crate::ffi::*;
use crate::utils;

pub struct Filter {
    ptr: *mut AVFilter,
}

impl Filter {
    pub unsafe fn wrap(ptr: *mut AVFilter) -> Self {
        Filter { ptr }
    }

    pub unsafe fn as_ptr(&self) -> *const AVFilter {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVFilter {
        self.ptr
    }
}

impl Filter {
    pub fn name(&self) -> &str {
        unsafe { utils::str_from_c_ptr((*self.as_ptr()).name) }
    }

    pub fn description(&self) -> Option<&str> {
        unsafe { utils::optional_str_from_c_ptr((*self.as_ptr()).description) }
    }

    pub fn inputs(&self) -> Option<PadIter> {
        unsafe {
            let ptr = (*self.as_ptr()).inputs;

            if ptr.is_null() {
                None
            } else {
                #[cfg(feature = "ffmpeg_5_0")]
                let count = (*self.as_ptr()).nb_inputs as isize;
                #[cfg(not(feature = "ffmpeg_5_0"))]
                let count = avfilter_pad_count(ptr) as isize;

                Some(PadIter::new(ptr, count))
            }
        }
    }

    pub fn outputs(&self) -> Option<PadIter> {
        unsafe {
            let ptr = (*self.as_ptr()).outputs;

            if ptr.is_null() {
                None
            } else {
                #[cfg(feature = "ffmpeg_5_0")]
                let count = (*self.as_ptr()).nb_outputs as isize;
                #[cfg(not(feature = "ffmpeg_5_0"))]
                let count = avfilter_pad_count(ptr) as isize;

                Some(PadIter::new(ptr, count))
            }
        }
    }

    pub fn flags(&self) -> Flags {
        unsafe { Flags::from_bits_truncate((*self.as_ptr()).flags) }
    }
}

pub struct PadIter<'a> {
    ptr: *const AVFilterPad,
    count: isize,
    cur: isize,

    _marker: PhantomData<&'a ()>,
}

impl<'a> PadIter<'a> {
    pub fn new(ptr: *const AVFilterPad, count: isize) -> Self {
        PadIter {
            ptr,
            count,
            cur: 0,
            _marker: PhantomData,
        }
    }
}

impl<'a> Iterator for PadIter<'a> {
    type Item = Pad<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.cur >= self.count {
                return None;
            }

            let pad = Pad::wrap(self.ptr, self.cur);
            self.cur += 1;

            Some(pad)
        }
    }
}
