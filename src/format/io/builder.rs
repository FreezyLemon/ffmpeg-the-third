#[path = "builder_helpers.rs"]
pub mod builder_helpers;

use super::{InputContext, OutputContext};

use libc::{c_int, c_uchar, c_void};
use std::ptr::NonNull;

use crate::ffi::*;
use crate::Error;

#[cfg(feature = "ffmpeg_7_0")]
pub type WriteFnBuf = *const u8;
#[cfg(not(feature = "ffmpeg_7_0"))]
pub type WriteFnBuf = *mut u8;

/// Helper struct to make preparing a custom [`AVIOContext`][crate::ffi::AVIOContext]
/// easier.
///
/// # Note about `user_data` and how it is used
///
/// `user_data` is any data that can be used for reading, writing or seeking.
///
/// It will be passed to all read/write/seek functions in the form of an opaque
/// (`*mut c_void`) pointer and can be used to access file streams, network streams
/// and any other data source or sink.
///
/// The recommended way of using `user_data` is supplying a value implementing
/// [`Read`][std::io::Read] and/or [`Write`][std::io::Write] and/or
/// [`Seek`][std::io::Seek] and calling the respective `with_default_` functions:
///
/// ```no_run
/// use std::fs::File;
/// use ffmpeg_the_third::format::io::IOContextBuilder;
///
/// let file = File::open("my_file.mp4")?;
/// // Not using BufReader because IOContext has its own buffer
/// let input_ctx = IOContextBuilder::new_input(4096, Box::new(file))
///     .with_default_read_fn()
///     .build()
///     .expect("can allocate memory");
///
/// # Ok::<(), std::io::Error>(())
/// ```
///
/// # Implementing your own read/write/seek callback
///
/// It's also possible to supply your own function pointer.
///
/// Since this builder uses a Box<T> to supply `opaque`, you can assume it is
/// non-null and aligned (i.e. safe to dereference and use like a &mut T or
/// even owned T).
///
/// If you are writing a read/write callback, it *should* be safe to call
/// [std::slice::from_raw_parts] or [std::slice::from_raw_parts_mut] with
/// `buf` and `buf_size`. See the default [`read_fn`][self::builder_helpers::read_fn]
/// and [`write_fn`][self::builder_helpers::write_fn] code as an example.
///
/// ```no_run
/// use std::fs::File;
/// use std::io::{self, Seek, SeekFrom};
/// use libc::{c_void, c_int};
/// use ffmpeg_the_third::format::io::IOContextBuilder;
/// use ffmpeg_the_third::ffi::AVERROR;
///
/// unsafe extern "C" fn seek_callback(opaque: *mut c_void, _offset: i64, _whence: c_int) -> i64 {
///     // IOContextBuilder ensures the `opaque` pointer is non-null and
///     // can be used as a mutable reference.
///     let file: &mut File = unsafe { (opaque as *mut File).as_mut().unwrap() };
///
///     // Ignore `offset` and `whence` parameters for brevity, just seek 10 bytes ahead.
///     match file.seek(SeekFrom::Current(10)) {
///         Ok(pos) => pos as i64,
///         Err(_) => AVERROR(libc::EINVAL) as i64,
///     }
/// }
///
/// fn main() -> io::Result<()> {
///     let file = File::open("my_other_file.mp3")?;
///
///     // SAFETY: Because with_seek_fn accepts unsafe functions,
///     //         I have ensured all safety invariants have been met.
///     let input_ctx = unsafe {
///         IOContextBuilder::new_input(4096, Box::new(file))
///             .with_default_read_fn()
///             .with_seek_fn(seek_callback)
///             .build()
///             .expect("can allocate memory")
///     };
///
///     Ok(())
/// }
/// ```
pub struct IOContextBuilder<T, const IS_OUTPUT: bool> {
    buf_size: usize,
    user_data: Box<T>,
    read_fn:
        Option<unsafe extern "C" fn(opaque: *mut c_void, buf: *mut u8, buf_size: c_int) -> c_int>,
    write_fn: Option<
        unsafe extern "C" fn(opaque: *mut c_void, buf: WriteFnBuf, buf_size: c_int) -> c_int,
    >,
    seek_fn: Option<unsafe extern "C" fn(opaque: *mut c_void, offset: i64, whence: c_int) -> i64>,
}

impl<T, const IS_OUTPUT: bool> IOContextBuilder<T, IS_OUTPUT> {
    fn new(buf_size: usize, user_data: Box<T>) -> Self {
        Self {
            buf_size,
            user_data,
            read_fn: None,
            write_fn: None,
            seek_fn: None,
        }
    }

    /// Supply your own callback to read from a source (i.e. `user_data`)
    /// into the AVIO buffer.
    ///
    /// See the [`builder struct documentation`][IOContextBuilder] for more details.
    pub unsafe fn with_read_fn(
        mut self,
        read_fn: unsafe extern "C" fn(opaque: *mut c_void, buf: *mut u8, buf_size: c_int) -> c_int,
    ) -> Self {
        self.read_fn = Some(read_fn);
        self
    }

    /// Supply your own callback to read from the AVIO buffer into
    /// a destination (i.e. `user_data`).
    ///
    /// See the [`builder struct documentation`][IOContextBuilder] for more details.
    pub unsafe fn with_write_fn(
        mut self,
        write_fn: unsafe extern "C" fn(
            opaque: *mut c_void,
            buf: WriteFnBuf,
            buf_size: c_int,
        ) -> c_int,
    ) -> Self {
        self.write_fn = Some(write_fn);
        self
    }

    /// Supply your own callback that allows seeking the custom source or destination
    /// (i.e. `user_data`).
    ///
    /// See the [`builder struct documentation`][IOContextBuilder] for more details.
    pub unsafe fn with_seek_fn(
        mut self,
        seek_fn: unsafe extern "C" fn(opaque: *mut c_void, offset: i64, whence: c_int) -> i64,
    ) -> Self {
        self.seek_fn = Some(seek_fn);
        self
    }

    fn build_inner(self) -> Result<NonNull<AVIOContext>, Error> {
        let Ok(int_size) = c_int::try_from(self.buf_size) else {
            return Err(Error::Other {
                errno: libc::EINVAL,
            });
        };

        let buf_ptr = unsafe { av_mallocz(self.buf_size) as *mut c_uchar };

        if buf_ptr.is_null() {
            return Err(Error::Other {
                errno: libc::ENOMEM,
            });
        }

        let ctx_ptr = unsafe {
            avio_alloc_context(
                buf_ptr,
                int_size,
                c_int::from(IS_OUTPUT),
                Box::into_raw(self.user_data) as *mut c_void,
                self.read_fn,
                self.write_fn,
                self.seek_fn,
            )
        };

        NonNull::new(ctx_ptr).ok_or(Error::Other {
            errno: libc::ENOMEM,
        })
    }
}

impl<T> IOContextBuilder<T, false> {
    /// Creates a new [`IOContextBuilder`] with the provided buffer size and
    /// user data.
    pub fn new_input(buf_size: usize, user_data: Box<T>) -> IOContextBuilder<T, false> {
        IOContextBuilder::new(buf_size, user_data)
    }

    /// Consume this builder to generate a [`InputContext`].
    ///
    /// # Errors
    /// - `Error::Other` containing `ENOMEM` if any allocation returned a nullptr.
    pub fn build(self) -> Result<InputContext<T>, Error> {
        // SAFETY: `ptr` will be non-null and will have exclusive ownership.
        unsafe {
            self.build_inner()
                .map(|ptr| InputContext::from_raw(ptr.as_ptr()).expect("non-null"))
        }
    }
}

impl<T> IOContextBuilder<T, true> {
    /// Creates a new [`IOContextBuilder`] with the provided buffer size and
    /// user data.
    pub fn new_output(buf_size: usize, user_data: Box<T>) -> IOContextBuilder<T, true> {
        IOContextBuilder::new(buf_size, user_data)
    }

    /// Consume this builder to generate a [`OutputContext`].
    ///
    /// # Errors
    /// - `Error::Other` containing `ENOMEM` if any allocation returned a nullptr.
    pub fn build(self) -> Result<OutputContext<T>, Error> {
        // SAFETY: `ptr` will be non-null and will have exclusive ownership.
        unsafe {
            self.build_inner()
                .map(|ptr| OutputContext::from_raw(ptr.as_ptr()).expect("non-null"))
        }
    }
}
