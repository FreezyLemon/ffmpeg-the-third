use libc::c_void;
use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::ffi::*;

use super::IOContextBuilder;
use crate::{AsMutPtr, AsPtr};

pub struct OutputContext<T> {
    ptr: NonNull<AVIOContext>,
    _marker: PhantomData<T>,
}

impl<T> OutputContext<T> {
    /// Creates an [`IOContextBuilder`] with the given buffer size and user data.
    ///
    /// See [`IOContextBuilder`] for more information on how to create an IO context.
    pub fn builder(buf_size: usize, user_data: Box<T>) -> IOContextBuilder<T, true> {
        IOContextBuilder::new_output(buf_size, user_data)
    }

    /// # Safety
    /// - `ptr` must be null or valid,
    /// - if `ptr` is non-null, `(*ptr).opaque` must be either null or
    ///   a valid *mut T with exclusive ownership.
    pub unsafe fn from_raw(ptr: *mut AVIOContext) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self {
            ptr,
            _marker: PhantomData,
        })
    }
}

impl<T> Drop for OutputContext<T> {
    fn drop(&mut self) {
        unsafe {
            // SAFETY: Ensured by `Self::from_raw` to be safe.
            drop(Box::from_raw((*self.as_mut_ptr()).opaque as *mut T));
            (*self.as_mut_ptr()).opaque = std::ptr::null_mut();

            av_freep(&mut (*self.as_mut_ptr()).buffer as *mut *mut _ as *mut c_void);
            avio_context_free(&mut self.as_mut_ptr());
        }
    }
}

impl<T> AsPtr<AVIOContext> for OutputContext<T> {
    fn as_ptr(&self) -> *const AVIOContext {
        self.ptr.as_ptr()
    }
}

impl<T> AsMutPtr<AVIOContext> for OutputContext<T> {
    fn as_mut_ptr(&mut self) -> *mut AVIOContext {
        self.ptr.as_ptr()
    }
}
