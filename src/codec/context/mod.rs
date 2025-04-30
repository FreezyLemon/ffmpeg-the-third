pub mod decoding;
pub mod encoding;

mod common;
mod closed;

use std::marker::PhantomData;
use std::ptr::NonNull;

use super::codec::*;
use crate::ffi::*;
use crate::option;
use crate::{AsMutPtr, AsPtr};
use crate::Codec;

pub struct Context<Action, CodecType, State = Closed>
{
    ptr: NonNull<AVCodecContext>,
    _marker: PhantomData<(Action, Codec<CodecType>, State)>,
}

pub struct Decoding;
pub struct Encoding;

pub struct Closed;
pub struct Opened;

unsafe impl<A, C, S> Send for Context<A, C, S> {}

fn new_context<A, C, S>(codec: Codec<C>) -> Context<A, C, S> {
    let ptr = unsafe { avcodec_alloc_context3(codec.as_ptr()) };

    Context {
        ptr: NonNull::new(ptr).expect("can allocate codec context"),
        _marker: PhantomData,
    }
}

impl<A, C, S> Context<A, C, S> {
    pub fn new_video_encoder(codec: Codec) -> Option<Context<Encoding, VideoType>> {
        if codec.is_encoder() {
            codec.video().map(new_context)
        } else {
            None
        }
    }

    pub fn new_video_decoder(codec: Codec) -> Option<Context<Decoding, VideoType>> {
        if codec.is_decoder() {
            codec.video().map(new_context)
        } else {
            None
        }
    }

    pub fn new_audio_encoder(codec: Codec) -> Option<Context<Encoding, AudioType>> {
        if codec.is_encoder() {
            codec.audio().map(new_context)
        } else {
            None
        }
    }

    pub fn new_audio_decoder(codec: Codec) -> Option<Context<Decoding, AudioType>> {
        if codec.is_decoder() {
            codec.audio().map(new_context)
        } else {
            None
        }
    }

    pub fn new_data_encoder(codec: Codec) -> Option<Context<Encoding, DataType>> {
        if codec.is_encoder() {
            codec.data().map(new_context)
        } else {
            None
        }
    }

    pub fn new_data_decoder(codec: Codec) -> Option<Context<Decoding, DataType>> {
        if codec.is_decoder() {
            codec.data().map(new_context)
        } else {
            None
        }
    }

    pub fn new_subtitle_encoder(codec: Codec) -> Option<Context<Encoding, SubtitleType>> {
        if codec.is_encoder() {
            codec.subtitle().map(new_context)
        } else {
            None
        }
    }

    pub fn new_subtitle_decoder(codec: Codec) -> Option<Context<Decoding, SubtitleType>> {
        if codec.is_decoder() {
            codec.subtitle().map(new_context)
        } else {
            None
        }
    }

    pub fn new_attachment_encoder(codec: Codec) -> Option<Context<Encoding, AttachmentType>> {
        if codec.is_encoder() {
            codec.attachment().map(new_context)
        } else {
            None
        }
    }

    pub fn new_attachment_decoder(codec: Codec) -> Option<Context<Decoding, AttachmentType>> {
        if codec.is_decoder() {
            codec.attachment().map(new_context)
        } else {
            None
        }
    }

    // pub fn from_parameters<P: AsPtr<AVCodecParameters>>(parameters: P) -> Result<Self, Error> {
    //     let mut context = Self::new();

    //     unsafe {
    //         match avcodec_parameters_to_context(context.as_mut_ptr(), parameters.as_ptr()) {
    //             e if e < 0 => Err(Error::from(e)),
    //             _ => Ok(context),
    //         }
    //     }
    // }
}

impl<A, C, S> Drop for Context<A, C, S> {
    fn drop(&mut self) {
        unsafe {
            avcodec_free_context(&mut self.as_mut_ptr());
        }
    }
}

/// `AVCodecContext` in `Context` is the target of `option` operations.
impl<A, C, S> AsPtr<AVCodecContext> for Context<A, C, S> {
    fn as_ptr(&self) -> *const AVCodecContext {
        self.ptr.as_ptr()
    }
}

impl<A, C, S> AsMutPtr<AVCodecContext> for Context<A, C, S> {
    fn as_mut_ptr(&mut self) -> *mut AVCodecContext {
        self.ptr.as_ptr()
    }
}

impl<A, C, S> option::Settable<AVCodecContext> for Context<A, C, S> {}
