use std::ffi::CString;
use std::ops::Index;
use std::ptr;
use std::slice;

use crate::ffi::*;
use crate::utils;
use libc::{c_int, c_void};
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Sample {
    None,

    U8(Type),
    I16(Type),
    I32(Type),
    I64(Type),
    F32(Type),
    F64(Type),
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Type {
    Packed,
    Planar,
}

impl Sample {
    #[inline]
    pub fn name(&self) -> &'static str {
        unsafe { utils::str_from_c_ptr(av_get_sample_fmt_name((*self).into())) }
    }

    #[inline]
    pub fn packed(&self) -> Self {
        unsafe { Sample::from(av_get_packed_sample_fmt((*self).into())) }
    }

    #[inline]
    pub fn planar(&self) -> Self {
        unsafe { Sample::from(av_get_planar_sample_fmt((*self).into())) }
    }

    #[inline]
    pub fn is_planar(&self) -> bool {
        unsafe { av_sample_fmt_is_planar((*self).into()) == 1 }
    }

    #[inline]
    pub fn is_packed(&self) -> bool {
        !self.is_planar()
    }

    #[inline]
    pub fn bytes(&self) -> usize {
        unsafe { av_get_bytes_per_sample((*self).into()) as usize }
    }

    #[inline]
    pub fn buffer(&self, channels: u16, samples: usize, align: bool) -> Buffer {
        Buffer::new(*self, channels, samples, align)
    }
}

impl From<AVSampleFormat> for Sample {
    #[inline]
    fn from(value: AVSampleFormat) -> Self {
        use AVSampleFormat as AV;

        match value {
            AV::AV_SAMPLE_FMT_NONE => Self::None,

            AV::AV_SAMPLE_FMT_U8 => Self::U8(Type::Packed),
            AV::AV_SAMPLE_FMT_S16 => Self::I16(Type::Packed),
            AV::AV_SAMPLE_FMT_S32 => Self::I32(Type::Packed),
            AV::AV_SAMPLE_FMT_S64 => Self::I64(Type::Packed),
            AV::AV_SAMPLE_FMT_FLT => Self::F32(Type::Packed),
            AV::AV_SAMPLE_FMT_DBL => Self::F64(Type::Packed),

            AV::AV_SAMPLE_FMT_U8P => Self::U8(Type::Planar),
            AV::AV_SAMPLE_FMT_S16P => Self::I16(Type::Planar),
            AV::AV_SAMPLE_FMT_S32P => Self::I32(Type::Planar),
            AV::AV_SAMPLE_FMT_S64P => Self::I64(Type::Planar),
            AV::AV_SAMPLE_FMT_FLTP => Self::F32(Type::Planar),
            AV::AV_SAMPLE_FMT_DBLP => Self::F64(Type::Planar),

            AV::AV_SAMPLE_FMT_NB => Self::None,

            #[cfg(feature = "non-exhaustive-enums")]
            _ => unimplemented!(),
        }
    }
}

impl From<&'static str> for Sample {
    #[inline]
    fn from(value: &'static str) -> Self {
        unsafe {
            let value = CString::new(value).unwrap();

            Sample::from(av_get_sample_fmt(value.as_ptr()))
        }
    }
}

impl From<Sample> for AVSampleFormat {
    #[inline]
    fn from(value: Sample) -> AVSampleFormat {
        use Sample as SF;

        match value {
            SF::None => Self::AV_SAMPLE_FMT_NONE,

            SF::U8(Type::Packed) => Self::AV_SAMPLE_FMT_U8,
            SF::I16(Type::Packed) => Self::AV_SAMPLE_FMT_S16,
            SF::I32(Type::Packed) => Self::AV_SAMPLE_FMT_S32,
            SF::I64(Type::Packed) => Self::AV_SAMPLE_FMT_S64,
            SF::F32(Type::Packed) => Self::AV_SAMPLE_FMT_FLT,
            SF::F64(Type::Packed) => Self::AV_SAMPLE_FMT_DBL,

            SF::U8(Type::Planar) => Self::AV_SAMPLE_FMT_U8P,
            SF::I16(Type::Planar) => Self::AV_SAMPLE_FMT_S16P,
            SF::I32(Type::Planar) => Self::AV_SAMPLE_FMT_S32P,
            SF::I64(Type::Planar) => Self::AV_SAMPLE_FMT_S64P,
            SF::F32(Type::Planar) => Self::AV_SAMPLE_FMT_FLTP,
            SF::F64(Type::Planar) => Self::AV_SAMPLE_FMT_DBLP,
        }
    }
}

pub struct Buffer {
    pub format: Sample,
    pub channels: u16,
    pub samples: usize,
    pub align: bool,

    buffer: *mut *mut u8,
    size: c_int,
}

impl Buffer {
    #[inline]
    pub fn size(format: Sample, channels: u16, samples: usize, align: bool) -> usize {
        unsafe {
            av_samples_get_buffer_size(
                ptr::null_mut(),
                i32::from(channels),
                samples as c_int,
                format.into(),
                !align as c_int,
            ) as usize
        }
    }

    #[inline]
    pub fn new(format: Sample, channels: u16, samples: usize, align: bool) -> Self {
        unsafe {
            let mut buf = Buffer {
                format,
                channels,
                samples,
                align,

                buffer: ptr::null_mut(),
                size: 0,
            };

            av_samples_alloc_array_and_samples(
                &mut buf.buffer,
                &mut buf.size,
                i32::from(channels),
                samples as c_int,
                format.into(),
                !align as c_int,
            );

            buf
        }
    }
}

impl Index<usize> for Buffer {
    type Output = [u8];

    #[inline]
    fn index(&self, index: usize) -> &[u8] {
        if index >= self.samples {
            panic!("out of bounds");
        }

        unsafe { slice::from_raw_parts(*self.buffer.add(index), self.size as usize) }
    }
}

impl Clone for Buffer {
    #[inline]
    fn clone(&self) -> Self {
        let mut buf = Buffer::new(self.format, self.channels, self.samples, self.align);
        buf.clone_from(self);

        buf
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        unsafe {
            av_samples_copy(
                self.buffer,
                source.buffer as *const *mut u8,
                0,
                0,
                source.samples as c_int,
                i32::from(source.channels),
                source.format.into(),
            );
        }
    }
}

impl Drop for Buffer {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            av_freep(self.buffer as *mut c_void);
        }
    }
}
