mod decoding;
mod encoding;

use std::marker::PhantomData;
use std::ptr::{self, NonNull};

use super::{threading, Compliance, Debug, Flags, Id, Parameters};
use crate::codec::codec::*;
use crate::media;
use crate::{ffi::*, Dictionary};
use crate::{Codec, Error};
use libc::{c_int, c_uint};

#[derive(Debug)]
pub struct Context<Action, CodecType, State = Closed>
where
    Action: self::Action,
    CodecType: crate::codec::CodecType,
    State: self::State,
{
    ptr: NonNull<AVCodecContext>,
    _marker: PhantomData<(Action, Codec<CodecType>, State)>,
}

unsafe impl<A: Action, T: CodecType, S: State> Send for Context<A, T, S> {}

// TODO: Seal with traits?
mod private {
    pub trait Sealed {}

    impl Sealed for super::Decoding {}
    impl Sealed for super::Encoding {}
    impl Sealed for super::Closed {}
    impl Sealed for super::Opened {}
}

pub trait Action: private::Sealed {}
pub trait State: private::Sealed {}

#[derive(Debug)]
pub struct Decoding;
#[derive(Debug)]
pub struct Encoding;

impl Action for Decoding {}
impl Action for Encoding {}

#[derive(Debug)]
pub struct Closed;
#[derive(Debug)]
pub struct Opened;

impl State for Closed {}
impl State for Opened {}

fn new_context<A: Action, T: CodecType, S: State>(codec: Codec<T>) -> Context<A, T, S> {
    let ptr = unsafe { avcodec_alloc_context3(codec.as_ptr()) };

    Context {
        ptr: NonNull::new(ptr).expect("can allocate codec context"),
        _marker: PhantomData,
    }
}

impl<A: Action, C: CodecType, S: State> Context<A, C, S> {
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

    // pub fn from_parameters<P: Into<Parameters>>(codec: Codec, parameters: P) -> Result<ContextNew, Error> {
    //     let parameters = parameters.into();
    //     let mut context = Self::new(codec);

    //     unsafe {
    //         match avcodec_parameters_to_context(context.as_mut_ptr(), parameters.as_ptr()) {
    //             e if e < 0 => Err(Error::from(e)),
    //             _ => Ok(context),
    //         }
    //     }
    // }

    pub fn as_ptr(&self) -> *const AVCodecContext {
        self.ptr.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut AVCodecContext {
        self.ptr.as_ptr()
    }

    pub fn codec(&self) -> Codec {
        unsafe { Codec::from_raw((*self.as_ptr()).codec).expect("Codec is set") }
    }

    pub fn medium(&self) -> media::Type {
        unsafe { media::Type::from((*self.as_ptr()).codec_type) }
    }

    pub fn id(&self) -> Id {
        unsafe { Id::from((*self.as_ptr()).codec_id) }
    }

    // TODO: Improve API
    pub fn set_codec_tag(&mut self, fourcc: u32) {
        unsafe {
            (*self.as_mut_ptr()).codec_tag = fourcc;
        }
    }

    pub fn flags(&self) -> Flags {
        unsafe { Flags::from_bits_truncate((*self.as_ptr()).flags as c_uint) }
    }

    pub fn set_flags(&mut self, value: Flags) {
        unsafe {
            (*self.as_mut_ptr()).flags = value.bits() as c_int;
        }
    }

    pub fn set_compliance(&mut self, value: Compliance) {
        unsafe {
            (*self.as_mut_ptr()).strict_std_compliance = value.into();
        }
    }

    pub fn set_debug(&mut self, value: Debug) {
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

    pub fn set_parameters<P: Into<Parameters>>(&mut self, parameters: P) -> Result<(), Error> {
        let parameters = parameters.into();

        unsafe {
            match avcodec_parameters_to_context(self.as_mut_ptr(), parameters.as_ptr()) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(()),
            }
        }
    }
}

impl<A: Action, C: CodecType> Context<A, C, Closed> {
    pub fn open(mut self) -> Result<Context<A, C, Opened>, Error> {
        let ret = unsafe {
            // TODO: support third param (options)
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

    pub fn open_with(mut self, options: Dictionary) -> Result<Context<A, C, Opened>, Error> {
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

impl<A: Action, C: CodecType, S: State> Drop for Context<A, C, S> {
    fn drop(&mut self) {
        unsafe {
            avcodec_free_context(&mut self.as_mut_ptr());
        }
    }
}
