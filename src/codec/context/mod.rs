mod decoding;
mod encoding;

use std::marker::PhantomData;
use std::ptr::NonNull;

use super::codec::*;
use super::{threading, Compliance, Debug, Flags, Id};
use crate::ffi::*;
use crate::media;
use crate::option;
use crate::{AsMutPtr, AsPtr};
use crate::{Codec, Error};
use libc::c_int;

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

    pub fn codec(&self) -> Option<Codec> {
        unsafe { Codec::from_raw((*self.as_ptr()).codec) }
    }

    pub fn medium(&self) -> media::Type {
        unsafe { media::Type::from((*self.as_ptr()).codec_type) }
    }

    pub fn set_flags(&mut self, value: Flags) {
        unsafe {
            (*self.as_mut_ptr()).flags = value.bits() as c_int;
        }
    }

    pub fn id(&self) -> Id {
        unsafe { Id::from((*self.as_ptr()).codec_id) }
    }

    pub fn compliance(&mut self, value: Compliance) {
        unsafe {
            (*self.as_mut_ptr()).strict_std_compliance = value.into();
        }
    }

    pub fn debug(&mut self, value: Debug) {
        unsafe {
            (*self.as_mut_ptr()).debug = value.bits();
        }
    }

    pub fn set_threading(&mut self, config: threading::Config) {
        unsafe {
            (*self.as_mut_ptr()).thread_type = config.kind.into();
            (*self.as_mut_ptr()).thread_count = config.count as c_int;
            #[cfg(not(feature = "ffmpeg_6_0"))]
            {
                (*self.as_mut_ptr()).thread_safe_callbacks = i32::from(config.safe);
            }
        }
    }

    pub fn threading(&self) -> threading::Config {
        unsafe {
            threading::Config {
                kind: threading::Type::from((*self.as_ptr()).active_thread_type),
                count: (*self.as_ptr()).thread_count as usize,
                #[cfg(not(feature = "ffmpeg_6_0"))]
                safe: (*self.as_ptr()).thread_safe_callbacks != 0,
            }
        }
    }

    pub fn set_parameters<P: AsPtr<AVCodecParameters>>(
        &mut self,
        parameters: P,
    ) -> Result<(), Error> {
        unsafe {
            match avcodec_parameters_to_context(self.as_mut_ptr(), parameters.as_ptr()) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(()),
            }
        }
    }
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
