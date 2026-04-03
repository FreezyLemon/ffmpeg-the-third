use super::{IOContextBuilder, WriteFnBuf};
use crate::ffi::*;

use libc::{c_int, c_void};
use std::io::{Read, Seek, SeekFrom, Write};
use std::ptr::NonNull;

impl<R: Read, const IS_OUTPUT: bool> IOContextBuilder<R, IS_OUTPUT> {
    /// Registers a read function (`read_packet`) that is usable with any type
    /// implementing [`Read`][std::io::Read].
    ///
    /// If you want to register your own function pointer, see
    /// [`with_read_fn`][IOContextBuilder::with_read_fn].
    pub fn with_default_read_fn(self) -> Self {
        // SAFETY:
        // - `opaque` will be non-null and a valid `*mut R`
        // - `buf` will be allocated by `av_mallocz`, non-null, presumably valid
        //   and will have exclusive access to `buf_size` many bytes.
        unsafe { self.with_read_fn(read_fn::<R>) }
    }
}

impl<W: Write, const IS_OUTPUT: bool> IOContextBuilder<W, IS_OUTPUT> {
    /// Registers a write function (`write_packet`) that is usable with any type
    /// implementing [`Write`][std::io::Write].
    ///
    /// If you want to register your own function pointer, see
    /// [`with_write_fn`][IOContextBuilder::with_write_fn].
    pub fn with_default_write_fn(self) -> Self {
        // SAFETY:
        // - `opaque` will be non-null and a valid `*mut W`
        // - `buf` will be allocated by `av_mallocz`, non-null, presumably valid
        //   and will have exclusive access to `buf_size` many bytes.
        unsafe { self.with_write_fn(write_fn::<W>) }
    }
}

impl<S: Seek, const IS_OUTPUT: bool> IOContextBuilder<S, IS_OUTPUT> {
    /// Registers a seek function (`seek`) that is usable with any type
    /// implementing [`Seek`][std::io::Seek]. Does not support `AVSEEK_SIZE`
    /// at the moment.
    ///
    /// If you want to register your own function pointer, see
    /// [`with_seek_fn`][IOContextBuilder::with_seek_fn].
    pub fn with_default_seek_fn(self) -> Self {
        // SAFETY:
        // - `opaque` will be non-null and a valid `*mut S`
        unsafe { self.with_seek_fn(seek_fn::<S>) }
    }
}

/// # Safety
/// - `opaque` must be non-null and a valid `*mut R`,
/// - `buf` must be non-null and allocated by an `av_` allocation function,
/// - `buf` must be valid for reads and writes of at least `buf_size` many bytes,
/// - `buf` must have exclusive access to the underlying memory
/// - `buf_size` must never exceed `c_int::MAX`
/// - the size of the `buf` allocation must never exceed `isize::MAX`
pub unsafe extern "C" fn read_fn<R: Read>(
    opaque: *mut c_void,
    buf: *mut u8,
    buf_size: c_int,
) -> c_int {
    // buf_size should never overflow isize, and I think
    // we can statically guarantee this for all platforms
    const {
        assert!(
            size_of::<c_int>() <= size_of::<isize>(),
            "buf_size should never be able to overflow isize"
        )
    };

    // SAFETY: Ensured by caller to be safe.
    let read = unsafe { NonNull::new_unchecked(opaque as *mut R).as_mut() };
    let buf = unsafe { std::slice::from_raw_parts_mut(buf, buf_size as usize) };

    match read.read(buf) {
        Ok(0) => AVERROR_EOF,
        // SAFETY: Ensured by the caller (buf_size <= c_int::MAX).
        Ok(n) => unsafe { c_int::try_from(n).unwrap_unchecked() },
        Err(_) => AVERROR(libc::EIO),
    }
}

/// # Safety
/// - `opaque` must be non-null and a valid `*mut R`,
/// - `buf` must be non-null and allocated by an `av_` allocation function,
/// - `buf` must be valid for reads of at least `buf_size` many bytes,
/// - `buf` must have exclusive access to the underlying memory
/// - `buf_size` must never exceed `c_int::MAX`
/// - the size of the `buf` allocation must never exceed `isize::MAX`
pub unsafe extern "C" fn write_fn<W: Write>(
    opaque: *mut c_void,
    buf: WriteFnBuf,
    buf_size: c_int,
) -> c_int {
    // buf_size should never overflow isize, and I think
    // we can statically guarantee this for all platforms
    const {
        assert!(
            size_of::<c_int>() <= size_of::<isize>(),
            "buf_size should never be able to overflow isize"
        )
    };

    // SAFETY: Ensured by caller to be safe.
    let write = unsafe { NonNull::new_unchecked(opaque as *mut W).as_mut() };
    let buf = unsafe { std::slice::from_raw_parts(buf, buf_size as usize) };

    match write.write(buf) {
        // SAFETY: Ensured by the caller (buf_size <= c_int::MAX)
        Ok(n) => unsafe { c_int::try_from(n).unwrap_unchecked() },
        Err(_) => AVERROR(libc::EIO),
    }
}

/// # Safety
/// - `opaque` must be non-null and a valid `*mut S`,
pub unsafe extern "C" fn seek_fn<S: Seek>(opaque: *mut c_void, offset: i64, whence: c_int) -> i64 {
    // SAFETY: Ensured by the caller
    let seek = unsafe { NonNull::new_unchecked(opaque as *mut S).as_mut() };
    let f = match whence {
        libc::SEEK_SET => {
            SeekFrom::Start(u64::try_from(offset).expect("positive offset when seeking from start"))
        }
        libc::SEEK_CUR => SeekFrom::Current(offset),
        libc::SEEK_END => SeekFrom::End(offset),
        // maybe we can support this when Seek::stream_len() is stable
        AVSEEK_SIZE => return i64::from(AVERROR(libc::ENOSYS)),
        _ => return i64::from(AVERROR(libc::EINVAL)),
    };

    match seek.seek(f) {
        Ok(new_pos) => i64::try_from(new_pos).unwrap_or(i64::from(AVERROR(libc::ERANGE))),
        Err(_) => i64::from(AVERROR(libc::EIO)),
    }
}
