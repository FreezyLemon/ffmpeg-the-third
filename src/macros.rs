// TODO: Document macros
// TODO: Check if owned_wrapper macro is flexible enough for the various alloc/clone/drop functions

macro_rules! impl_owned_wrapper {
    ($wrapper_name:ident, $av_ty:ty, $av_alloc:ident, freep $av_drop:ident) => {
        impl_owned_wrapper!($wrapper_name, $av_ty);

        impl $wrapper_name {
            pub fn new() -> Self {
                let raw_ptr = unsafe { $av_alloc() };
                Self(std::ptr::NonNull::new(raw_ptr).expect("can allocate"))
            }
        }

        impl Drop for $wrapper_name {
            fn drop(&mut self) {
                unsafe {
                    $av_drop(&mut self.as_mut_ptr());
                }
            }
        }
    };
    ($wrapper_name:ident, $av_ty:ty, $av_alloc:ident, free $av_drop:ident) => {
        impl_owned_wrapper!($wrapper_name, $av_ty);

        impl $wrapper_name {
            pub fn new() -> Self {
                let raw_ptr = unsafe { $av_alloc() };
                Self(std::ptr::NonNull::new(raw_ptr).expect("can allocate"))
            }
        }

        impl Drop for $wrapper_name {
            fn drop(&mut self) {
                unsafe {
                    $av_drop(self.as_mut_ptr());
                }
            }
        }
    };
    ($wrapper_name:ident, $av_ty:ty) => {
        pub struct $wrapper_name(std::ptr::NonNull<$av_ty>);

        impl $wrapper_name {
            /// Returns the contained raw pointer. Guaranteed to be non-null.
            pub fn as_ptr(&self) -> *const $av_ty {
                self.0.as_ptr()
            }

            /// Returns the contained mutable raw pointer. Guaranteed to be non-null.
            pub fn as_mut_ptr(&mut self) -> *mut $av_ty {
                unsafe { self.0.as_mut() }
            }
        }
    };
}

macro_rules! impl_ref_wrapper {
    ($wrapper_name:ident, $av_ty:ty) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $wrapper_name<'a>(
            std::ptr::NonNull<$av_ty>,
            std::marker::PhantomData<&'a $av_ty>,
        );

        impl<'a> $wrapper_name<'a> {
            /// Create a new [`
            #[doc = stringify!($wrapper_name)]
            /// `] from a raw pointer.
            ///
            /// Returns `None` is the pointer is null.
            ///
            /// # Safety
            ///
            /// <TODO: mention shared reference>
            /// Callers must ensure that the pointer is either null or is valid.
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

macro_rules! impl_mut_wrapper {
    ($wrapper_name:ident, $av_ty:ty) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $wrapper_name<'a>(
            std::ptr::NonNull<$av_ty>,
            std::marker::PhantomData<&'a mut $av_ty>,
        );

        impl<'a> $wrapper_name<'a> {
            pub unsafe fn from_ptr(ptr: *mut $av_ty) -> Option<Self> {
                std::ptr::NonNull::new(ptr).map(|ptr| Self(ptr, std::marker::PhantomData))
            }

            /// Returns the contained raw pointer. Guaranteed to be non-null.
            pub fn as_ptr(&self) -> *const $av_ty {
                self.0.as_ptr()
            }

            /// Returns the contained mutable raw pointer. Guaranteed to be non-null.
            pub fn as_mut_ptr(&mut self) -> *mut $av_ty {
                unsafe { self.0.as_mut() }
            }
        }

        impl<'a> std::fmt::Pointer for $wrapper_name<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::Pointer::fmt(&self.as_ptr(), f)
            }
        }
    };
}

macro_rules! impl_getter_simple {
    ($fn_name:ident() -> $ty:ty; $av_field:ident) => {
        pub fn $fn_name(&self) -> $ty {
            unsafe { (*self.as_ptr()).$av_field as $ty }
        }
    };
}

macro_rules! impl_setter_simple {
    ($fn_name:ident($ty:ty); $av_field:ident: $av_ty:ty) => {
        pub fn $fn_name(&mut self, value: $ty) {
            unsafe { (*self.as_mut_ptr()).$av_field = value as $av_ty }
        }
    };
}

macro_rules! impl_field_string {
    ($fn_name:ident, $av_field:ident) => {
        pub fn $fn_name(&self) -> &str {
            unsafe {
                std::str::from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).$av_field).to_bytes())
            }
        }
    };
    (optional $fn_name:ident, $av_field:ident) => {
        pub fn $fn_name(&self) -> Option<&str> {
            unsafe {
                let ptr = (*self.as_ptr()).$av_field;
                if ptr.is_null() {
                    None
                } else {
                    Some(std::str::from_utf8_unchecked(
                        CStr::from_ptr(ptr).to_bytes(),
                    ))
                }
            }
        }
    };
}

macro_rules! impl_getter_into {
    ($fn_name:ident() -> $ty:ty; $av_field:ident) => {
        pub fn $fn_name(&self) -> $ty {
            unsafe { (*self.as_ptr()).$av_field.into() }
        }
    };
}

macro_rules! impl_setter_into {
    ($fn_name:ident($ty:ty); $av_field:ident) => {
        pub fn $fn_name<T: Into<$ty>>(&mut self, value: T) {
            unsafe { (*self.as_mut_ptr()).$av_field = value.into().into() }
        }
    };
}

macro_rules! impl_for_one {
    // ref/mut with lifetime
    ($wrapper:ident, $lt:lifetime, $func:item) => {
        impl<$lt> $wrapper<$lt> {
            $func
        }
    };
    // owned without lifetime
    ($wrapper:ident, $func:item) => {
        impl $wrapper {
            $func
        }
    };
}

macro_rules! impl_for_many {
    { impl for $($wrapper:ident$(<$lt:lifetime>)?),+ {} } => {};
    {
        impl for $($wrapper:ident$(<$lt:lifetime>)?),+ {
            $func:item
            $($tt:tt)*
        }
    } => {
        $(
            $crate::macros::impl_for_one!($wrapper$(, $lt)?, $func);
        )+

        impl_for_many!{
            impl for $($wrapper$(<$lt>)?),+ {
                $($tt)*
            }
        }
    };
}

// TODO: Complete this macro
macro_rules! impl_ffmpeg_list_iterator {
    ($iter_name:ident, $av_ty:ty, $ref_ty:ty) => {
        pub struct $iter_name<'a> {
            curr: *const $av_ty,
            remaining: usize,
            _marker: std::marker::PhantomData<&'a $av_ty>,
        }

        impl<'a> $iter_name<'a> {
            /// Create a new iterator over the
            #[doc = stringify!($ref_ty)]
            /// contained in a packet or stream.
            ///
            /// # Safety
            ///
            /// Callers must ensure that the memory pointed to by `ptr` is valid for at
            /// least `count` read accesses of
            #[doc = stringify!($av_ty)]
            /// .
            ///
            /// Callers must also ensure that the lifetime associated with the new iterator
            /// is correct, i.e. that the previous safety invariant holds for the lifetime
            /// of the new iterator.
            pub unsafe fn new(ptr: *const $av_ty, count: libc::c_int) -> Self {
                Self {
                    curr: ptr,
                    remaining: usize::try_from(count).expect("count fits into usize"),
                    _marker: PhantomData,
                }
            }
        }

        impl<'a> Iterator for $iter_name<'a> {
            type Item = $ref_ty;

            fn next(&mut self) -> Option<<Self as Iterator>::Item> {
                if self.remaining == 0 {
                    return None;
                }
                self.remaining -= 1;

                // SAFETY: Guaranteed by Self::new
                unsafe {
                    let curr = <$ref_ty>::from_ptr(self.curr)
                        .expect(&format!("{} ptr is non-null", stringify!($av_ty)));

                    self.curr = self.curr.add(1);

                    Some(curr)
                }
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                (self.remaining as usize, Some(self.remaining as usize))
            }
        }

        impl<'a> FusedIterator for $iter_name<'a> {}
        impl<'a> ExactSizeIterator for $iter_name<'a> {}
    };
}

pub(crate) use impl_ffmpeg_list_iterator;
pub(crate) use impl_field_string;
pub(crate) use impl_for_many;
pub(crate) use impl_for_one;
pub(crate) use impl_owned_wrapper;
pub(crate) use {impl_getter_into, impl_setter_into};
pub(crate) use {impl_getter_simple, impl_setter_simple};
pub(crate) use {impl_mut_wrapper, impl_ref_wrapper};
