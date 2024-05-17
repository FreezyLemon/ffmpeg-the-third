use std::borrow::Borrow;
use std::borrow::Cow;
use std::ffi::CString;

use crate::ffi::*;

pub struct Dictionary<'a>(Cow<'a, AVDictionary>);

impl<'a> Dictionary<'a> {
    pub fn empty() -> Self {
        todo!()
    }

    pub fn get(&self, key: &str) -> &str {
        unsafe {
            av_dict_get(self.0.as_ref(), CString::new(key).unwrap().into_raw(), std::ptr::null(), 0);
        }

        ""
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
