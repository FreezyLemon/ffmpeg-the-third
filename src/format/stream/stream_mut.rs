use std::mem;
use std::ops::Deref;

use super::Stream;
use crate::ffi::*;
use crate::format::context::common::Context;
use crate::AsPtr;
use crate::{codec, Dictionary, Rational};

pub struct StreamMut<'a> {
    context: &'a mut Context,
    index: usize,

    immutable: Stream<'a>,
}

impl<'a> StreamMut<'a> {
    pub unsafe fn wrap(context: &mut Context, index: usize) -> StreamMut {
        StreamMut {
            context: mem::transmute_copy(&context),
            index,

            immutable: Stream::wrap(mem::transmute_copy(&context), index),
        }
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVStream {
        *(*self.context.as_mut_ptr()).streams.add(self.index)
    }
}

impl<'a> StreamMut<'a> {
    pub fn set_time_base<R: Into<Rational>>(&mut self, value: R) {
        unsafe {
            (*self.as_mut_ptr()).time_base = value.into().into();
        }
    }

    pub fn set_rate<R: Into<Rational>>(&mut self, value: R) {
        unsafe {
            (*self.as_mut_ptr()).r_frame_rate = value.into().into();
        }
    }

    pub fn set_avg_frame_rate<R: Into<Rational>>(&mut self, value: R) {
        unsafe {
            (*self.as_mut_ptr()).avg_frame_rate = value.into().into();
        }
    }

    pub fn parameters_mut(&mut self) -> codec::ParametersMut {
        unsafe {
            codec::ParametersMut::from_raw((*self.as_mut_ptr()).codecpar)
                .expect("codecpar is non-null")
        }
    }

    pub fn set_parameters<P: AsPtr<AVCodecParameters>>(&mut self, parameters: P) {
        unsafe {
            avcodec_parameters_copy((*self.as_mut_ptr()).codecpar, parameters.as_ptr());
        }
    }

    pub fn copy_parameters_from_context(&mut self, ctx: &codec::Context) {
        unsafe {
            avcodec_parameters_from_context((*self.as_mut_ptr()).codecpar, ctx.as_ptr());
        }
    }

    pub fn set_metadata(&mut self, metadata: Dictionary) {
        unsafe {
            let metadata = metadata.disown();
            (*self.as_mut_ptr()).metadata = metadata;
        }
    }
}

impl<'a> Deref for StreamMut<'a> {
    type Target = Stream<'a>;

    fn deref(&self) -> &Self::Target {
        &self.immutable
    }
}
