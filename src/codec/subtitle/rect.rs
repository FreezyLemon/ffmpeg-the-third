use std::ffi::{CStr, CString};
// use std::ops::Deref;
use std::str::from_utf8_unchecked;

use libc::c_int;

use super::{Flags, Type};
use crate::ffi::*;
use crate::macros::{
    impl_for_many, impl_getter_into, impl_getter_simple, impl_mut_wrapper, impl_ref_wrapper,
    impl_setter_simple,
};
#[cfg(not(feature = "ffmpeg_5_0"))]
use crate::{format, Picture};

impl_ref_wrapper!(Rect, AVSubtitleRect);
impl_mut_wrapper!(RectMut, AVSubtitleRect);

impl_for_many! {
    impl for Rect<'a>, RectMut<'a> {
        impl_getter_into!(kind() -> Type; type_);

        pub fn flags(&self) -> Flags {
            unsafe {
                Flags::from_bits_truncate((*self.as_ptr()).flags)
            }
        }

        /// Tries to return the contained subtitle as a UTF-8 string slice.
        ///
        /// Returns `None` if the subtitle is a bitmap or has no defined type.
        pub fn as_string(&self) -> Option<&str> {
            unsafe {
                let text_ptr = match self.kind() {
                    Type::None => return None,
                    Type::Bitmap => return None,
                    Type::Text => (*self.as_ptr()).text,
                    Type::Ass => (*self.as_ptr()).ass,
                };

                Some(from_utf8_unchecked(CStr::from_ptr(text_ptr).to_bytes()))
            }
        }
    }
}

impl<'a> Rect<'a> {
    pub fn as_bitmap(&self) -> Option<Bitmap<'a>> {
        if self.kind() != Type::Bitmap {
            return None;
        }

        unsafe { Some(Bitmap::from_ptr(self.as_ptr()).unwrap()) }
    }

    pub fn as_text(&self) -> Option<Text<'a>> {
        if self.kind() != Type::Text {
            return None;
        }

        unsafe { Some(Text::from_ptr(self.as_ptr()).unwrap()) }
    }

    pub fn as_ass(&self) -> Option<Ass<'a>> {
        if self.kind() != Type::Ass {
            return None;
        }

        unsafe { Some(Ass::from_ptr(self.as_ptr()).unwrap()) }
    }
}

impl<'a> RectMut<'a> {
    pub fn as_bitmap(&mut self) -> Option<BitmapMut<'a>> {
        if self.kind() != Type::Bitmap {
            return None;
        }

        unsafe { Some(BitmapMut::from_ptr(self.as_mut_ptr()).unwrap()) }
    }

    pub fn as_text(&mut self) -> Option<TextMut<'a>> {
        if self.kind() != Type::Text {
            return None;
        }

        unsafe { Some(TextMut::from_ptr(self.as_mut_ptr()).unwrap()) }
    }

    pub fn as_ass(&mut self) -> Option<AssMut<'a>> {
        if self.kind() != Type::Ass {
            return None;
        }

        unsafe { Some(AssMut::from_ptr(self.as_mut_ptr()).unwrap()) }
    }
}

impl_ref_wrapper!(Bitmap, AVSubtitleRect);
impl_mut_wrapper!(BitmapMut, AVSubtitleRect);

impl<'a> Bitmap<'a> {
    impl_getter_simple!(x() -> u32; x);
    impl_getter_simple!(y() -> u32; y);
    impl_getter_simple!(width() -> u32; w);
    impl_getter_simple!(height() -> u32; h);
    impl_getter_simple!(colors() -> u32; nb_colors);

    // XXX: must split Picture and PictureMut
    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn picture(&self, format: format::Pixel) -> Picture<'a> {
        unsafe {
            Picture::wrap(
                &(*self.as_ptr()).pict as *const _ as *mut _,
                format,
                (*self.as_ptr()).w as u32,
                (*self.as_ptr()).h as u32,
            )
        }
    }
}

impl<'a> BitmapMut<'a> {
    impl_setter_simple!(set_x(u32); x: c_int);
    impl_setter_simple!(set_y(u32); y: c_int);
    impl_setter_simple!(set_width(u32); w: c_int);
    impl_setter_simple!(set_height(u32); h: c_int);
    impl_setter_simple!(set_colors(u32); nb_colors: c_int);
}

impl_ref_wrapper!(Text, AVSubtitleRect);
impl_mut_wrapper!(TextMut, AVSubtitleRect);

impl_for_many! {
    impl for Text<'a>, TextMut<'a> {
        pub fn get(&self) -> &str {
            unsafe {
                from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).text).to_bytes())
            }
        }
    }
}

impl<'a> TextMut<'a> {
    pub fn set(&mut self, value: &str) {
        let value = CString::new(value).unwrap();

        unsafe {
            (*self.as_mut_ptr()).text = av_strdup(value.as_ptr());
        }
    }
}

impl_ref_wrapper!(Ass, AVSubtitleRect);
impl_mut_wrapper!(AssMut, AVSubtitleRect);

impl_for_many! {
    impl for Ass<'a>, AssMut<'a> {
        pub fn get(&self) -> &str {
            unsafe {
                from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).ass).to_bytes())
            }
        }
    }
}

impl<'a> AssMut<'a> {
    pub fn set(&mut self, value: &str) {
        let value = CString::new(value).unwrap();

        unsafe {
            (*self.as_mut_ptr()).ass = av_strdup(value.as_ptr());
        }
    }
}
