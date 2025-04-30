use super::{Closed, Context, Opened};
use crate::ffi::*;
use crate::{AsMutPtr, Dictionary, Error};

use std::marker::PhantomData;
use std::ptr;

type OpenResult<Action, CodecType> = Result<Context<Action, CodecType, Opened>, Error>;

impl<A, T> Context<A, T, Closed> {
    pub fn open(mut self) -> OpenResult<A, T> {
        let ret = unsafe {
            avcodec_open2(self.as_mut_ptr(), ptr::null(), ptr::null_mut())
        };

        if ret < 0 {
            Err(Error::from(ret))
        } else {
            Ok(Context {
                ptr: self.ptr,
                _marker: PhantomData,
            })
        }
    }

    pub fn open_with(mut self, options: Dictionary) -> OpenResult<A, T> {
        let ret = unsafe {
            let mut opts = options.disown();
            let ret = avcodec_open2(self.as_mut_ptr(), ptr::null(), &mut opts);
            Dictionary::own(opts);
            ret
        };

        if ret < 0 {
            Err(Error::from(ret))
        } else {
            Ok(Context {
                ptr: self.ptr,
                _marker: PhantomData,
            })
        }
    }
}