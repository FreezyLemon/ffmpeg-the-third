pub mod slice;

pub mod conceal;
pub use self::conceal::Conceal;

use std::ffi::CString;

use crate::ffi::*;
use crate::codec::Id;
use crate::Codec;

pub fn find(id: Id) -> Option<Codec> {
    unsafe {
        let ptr = avcodec_find_decoder(id.into());
        Codec::from_raw(ptr)
    }
}

pub fn find_by_name(name: &str) -> Option<Codec> {
    unsafe {
        let name = CString::new(name).unwrap();
        let ptr = avcodec_find_decoder_by_name(name.as_ptr());

        Codec::from_raw(ptr)
    }
}
