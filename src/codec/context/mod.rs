use std::marker::PhantomData;
use std::ptr::{self, NonNull};

use super::{threading, Compliance, Debug, Flags, Id, Parameters};
use crate::{chroma, color, format, media, FieldOrder};
use crate::{ffi::*, Rational};
use crate::{Codec, Error};
use libc::{c_int, c_uint};

// Action = Decode/Encode
// (Type = Video/Audio)?
// State = Closed/Opened

pub type Decoder = Context<Decoding>;
pub type Encoder = Context<Encoding>;

#[derive(Debug)]
pub struct Context<Action, State = Closed> {
    ptr: NonNull<AVCodecContext>,
    _marker: PhantomData<(Action, State)>,
}

unsafe impl<A, S> Send for Context<A, S> {}

#[derive(Debug)]
pub struct Decoding;
#[derive(Debug)]
pub struct Encoding;

#[derive(Debug)]
pub struct Closed;
#[derive(Debug)]
pub struct Opened;

fn new_context<A, S>(codec: Codec) -> Context<A, S> {
    let ptr = unsafe { avcodec_alloc_context3(codec.as_ptr()) };

    Context {
        ptr: NonNull::new(ptr).expect("can allocate codec context"),
        _marker: PhantomData,
    }
}

impl Decoder {
    pub fn new(codec: Codec) -> Self {
        assert!(codec.is_decoder(), "Codec does not support decoding");
        new_context(codec)
    }
}

impl Encoder {
    pub fn new(codec: Codec) -> Self {
        assert!(codec.is_encoder(), "Codec does not support encoding");
        new_context(codec)
    }
}

impl<A, S> Context<A, S> {
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

impl<S> Context<Decoding, S> {
    // MUST be set
    pub fn set_time_base(&mut self, time_base: Rational) {
        unsafe {
            (*self.as_mut_ptr()).time_base = time_base.into();
        }
    }

    pub fn set_pkt_timebase(&mut self, pkt_timebase: Rational) {
        unsafe {
            (*self.as_mut_ptr()).pkt_timebase = pkt_timebase.into();
        }
    }

    // {0, 1} if unknown, otherwise signals framerate
    pub fn framerate(&self) -> Rational {
        unsafe { (*self.as_ptr()).framerate.into() }
    }

    // Number of frames delay IN ADDITION to a spec-conforming decoder
    // audio: number of samples the decoder needs to output before output is valid
    pub fn delay(&self) -> i32 {
        unsafe { (*self.as_ptr()).delay }
    }

    // required for some decoders, optional otherwise
    pub fn set_width(&mut self, width: i32) {
        unsafe {
            (*self.as_mut_ptr()).width = width as c_int;
        }
    }

    // required for some decoders, optional otherwise
    pub fn set_height(&mut self, height: i32) {
        unsafe {
            (*self.as_mut_ptr()).height = height as c_int;
        }
    }

    pub fn sample_aspect_ratio(&self) -> Rational {
        unsafe { (*self.as_ptr()).sample_aspect_ratio.into() }
    }

    pub fn pix_fmt(&self) -> format::Pixel {
        unsafe { (*self.as_ptr()).pix_fmt.into() }
    }

    pub fn set_pix_fmt(&mut self, pix_fmt: format::Pixel) {
        unsafe { (*self.as_mut_ptr()).pix_fmt = pix_fmt.into() }
    }

    // TODO: sw_pix_fmt?

    pub fn color_primaries(&self) -> color::Primaries {
        unsafe { (*self.as_ptr()).color_primaries.into() }
    }

    pub fn color_transfer(&self) -> color::TransferCharacteristic {
        unsafe { (*self.as_ptr()).color_trc.into() }
    }

    pub fn color_space(&self) -> color::Space {
        unsafe { (*self.as_ptr()).colorspace.into() }
    }

    pub fn color_range(&self) -> color::Range {
        unsafe { (*self.as_ptr()).color_range.into() }
    }

    pub fn set_color_range(&mut self, range: color::Range) {
        unsafe { (*self.as_mut_ptr()).color_range = range.into() }
    }

    pub fn chroma_sample_location(&self) -> chroma::Location {
        unsafe { (*self.as_ptr()).chroma_sample_location.into() }
    }

    pub fn set_field_order(&mut self, order: FieldOrder) {
        unsafe { (*self.as_mut_ptr()).field_order = order.into() }
    }

    pub fn reference_frames(&self) -> i32 {
        unsafe { (*self.as_ptr()).refs as i32 }
    }

    pub fn has_b_frames(&self) -> i32 {
        unsafe { (*self.as_ptr()).has_b_frames as i32 }
    }

    // TODO: slice_flags
    // TODO: draw_horiz_band
    // TODO: get_format
}

impl<S> Context<Encoding, S> {
    // Optional for CFR content
    pub fn set_framerate(&mut self, framerate: Rational) {
        unsafe {
            (*self.as_mut_ptr()).framerate = framerate.into();
        }
    }

    // Number of frames delay between encoder input and (spec-conforming) decoder output.
    // Unused for audio (see initial_padding)
    pub fn delay(&self) -> i32 {
        unsafe { (*self.as_ptr()).delay }
    }

    pub fn width(&self) -> i32 {
        unsafe { (*self.as_ptr()).width as i32 }
    }

    // required
    pub fn set_width(&mut self, width: i32) {
        unsafe { (*self.as_mut_ptr()).width = width as c_int }
    }

    pub fn height(&self) -> i32 {
        unsafe { (*self.as_ptr()).height as i32 }
    }

    // required
    pub fn set_height(&mut self, height: i32) {
        unsafe { (*self.as_mut_ptr()).height = height as c_int }
    }

    pub fn coded_width(&self) -> i32 {
        unsafe { (*self.as_ptr()).coded_width }
    }

    pub fn coded_height(&self) -> i32 {
        unsafe { (*self.as_ptr()).coded_height }
    }

    // optional? 0 if unknown
    // ratio=(width of pixel).div(height of pixel)
    pub fn set_sample_aspect_ratio(&mut self, ratio: Rational) {
        unsafe { (*self.as_mut_ptr()).sample_aspect_ratio = ratio.into() }
    }

    pub fn set_pix_fmt(&mut self, pix_fmt: format::Pixel) {
        unsafe { (*self.as_mut_ptr()).pix_fmt = pix_fmt.into() }
    }

    pub fn set_color_primaries(&mut self, primaries: color::Primaries) {
        unsafe { (*self.as_mut_ptr()).color_primaries = primaries.into() }
    }

    pub fn set_color_transfer(&mut self, transfer: color::TransferCharacteristic) {
        unsafe { (*self.as_mut_ptr()).color_trc = transfer.into() }
    }

    pub fn set_color_space(&mut self, space: color::Space) {
        unsafe { (*self.as_mut_ptr()).colorspace = space.into() }
    }

    // set = override default
    pub fn set_color_range(&mut self, range: color::Range) {
        unsafe { (*self.as_mut_ptr()).color_range = range.into() }
    }

    pub fn set_chroma_sample_location(&mut self, location: chroma::Location) {
        unsafe { (*self.as_mut_ptr()).chroma_sample_location = location.into() }
    }

    pub fn field_order(&self) -> FieldOrder {
        unsafe { (*self.as_ptr()).field_order.into() }
    }

    pub fn set_reference_frames(&mut self, frames: i32) {
        unsafe { (*self.as_mut_ptr()).refs = frames as c_int }
    }

    pub fn has_b_frames(&self) -> i32 {
        unsafe { (*self.as_ptr()).has_b_frames as i32 }
    }
}

impl Context<Closed> {
    // previous impl:
    // pub fn open(mut self) -> Result<Opened, Error> {
    //     unsafe {
    //         match avcodec_open2(self.as_mut_ptr(), ptr::null(), ptr::null_mut()) {
    //             0 => Ok(Opened(self)),
    //             e => Err(Error::from(e)),
    //         }
    //     }
    // }

    // pub fn open_with(mut self, options: Dictionary) -> Result<Encoder, Error> {
    //     unsafe {
    //         let mut opts = options.disown();
    //         let res = avcodec_open2(self.as_mut_ptr(), ptr::null(), &mut opts);

    //         Dictionary::own(opts);

    //         match res {
    //             0 => Ok(Encoder(self)),
    //             e => Err(Error::from(e)),
    //         }
    //     }
    // }

    pub fn open(mut self) -> Result<Context<Opened>, Error> {
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

    // pub fn asdf(&mut self) {
    //     unsafe {

    //     }
    // }
}

impl<A, S> Drop for Context<A, S> {
    fn drop(&mut self) {
        unsafe {
            avcodec_free_context(&mut self.as_mut_ptr());
        }
    }
}
