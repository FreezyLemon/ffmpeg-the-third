/// Implement a wrapper that acts as a read-only reference to an FFmpeg type with
/// some basic helper methods.
///
/// # Example
///
/// ```ignore
/// impl_ref_wrapper!(SideData, AVPacketSideData);
/// ```
///
/// expands to
///
/// ```ignore
/// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// pub struct SideData<'a>(
///     std::ptr::NonNull<AVPacketSideData>,
///     std::marker::PhantomData<&'a AVPacketSideData>,
/// );
///
/// impl<'a> SideData<'a> {
///     pub fn from_ptr(ptr: *const AVPacketSideData) -> Self {
///         std::ptr::NonNull::new(ptr as *mut _).map(|ptr| Self(ptr, std::marker::NonNull))
///     }
///
///     pub fn as_ptr(&self) -> *const AVPacketSideData {
///         self.0.as_ptr()
///     }
/// }
/// ```
macro_rules! impl_ref_wrapper {
    ($wrapper_name:ident, $av_ty:ty) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $wrapper_name<'a>(
            std::ptr::NonNull<$av_ty>,
            std::marker::PhantomData<&'a $av_ty>,
        );

        impl<'a> $wrapper_name<'a> {
            /// Create a new reference wrapper from a raw pointer.
            ///
            /// Returns `None` is the pointer is null.
            ///
            /// # Safety
            ///
            /// Callers must ensure that the pointer is either null or is valid.
            ///
            /// Callers must also ensure that Rust's rules for shared references are
            /// adhered to during the lifetime of the returned struct. For example,
            /// mutating the underlying FFmpeg struct through a raw pointer while a
            /// shared reference is being held results in undefined behavior.
            pub unsafe fn from_ptr(ptr: *const $av_ty) -> Option<Self> {
                std::ptr::NonNull::new(ptr as *mut _).map(|ptr| Self(ptr, std::marker::PhantomData))
            }

            /// Returns the contained raw pointer. Guaranteed to be non-null.
            pub fn as_ptr(&self) -> *const $av_ty {
                self.0.as_ptr()
            }
        }

        impl<'a> std::fmt::Pointer for $wrapper_name<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::Pointer::fmt(&self.as_ptr(), f)
            }
        }
    };
}

pub(crate) use impl_ref_wrapper;
