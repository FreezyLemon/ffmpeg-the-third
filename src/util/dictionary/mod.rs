use std::borrow::Borrow;
use std::borrow::Cow;
use std::ffi::CString;

use crate::ffi::*;

pub struct Dictionary<'a>(Option<Cow<'a, AVDictionary>>);

impl<'a> Dictionary<'a> {
    pub fn empty() -> Self {
        Self(None)
    }

    pub fn as_ptr(&self) -> *const AVDictionary {
        match &self.0 {
            Some(cow) => cow.as_ref(),
            None => std::ptr::null(),
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut AVDictionary {
        match &mut self.0 {
            Some(cow) => cow.to_mut(),
            None => std::ptr::null_mut(),
        }
    }

    pub fn get(&self, key: &str) -> &str {
        unsafe {
            av_dict_get(self.as_ptr(), CString::new(key).unwrap().into_raw(), std::ptr::null(), 0);
        }

        ""
    }
}

impl<'a> Drop for Dictionary<'a> {
    fn drop(&mut self) {
        if let Some(Cow::Owned(ref mut dict)) = self.0 {
            unsafe {
                av_dict_free(&mut (dict as _))
            }
        }
    }
}

impl<'a> From<AVDictionary> for Dictionary<'a> {
    fn from(value: AVDictionary) -> Self {
        Self(Cow::Owned(value))
    }
}

impl<'a> From<&'a AVDictionary> for Dictionary<'a> {
    fn from(value: &'a AVDictionary) -> Self {
        Self(Cow::Borrowed(value))
    }
}

impl<'a> Borrow<AVDictionary> for Dictionary<'a> {
    fn borrow(&self) -> &AVDictionary {
        &self.0
    }
}

mod iter;
pub use self::iter::Iter;

#[macro_export]
macro_rules! dict {
    ( $($key:expr => $value:expr),* $(,)*) => ({
            let mut dict = ::ffmpeg::Dictionary::new();

            $(
                dict.set($key, $value);
            )*

            dict
        }
    );
}
