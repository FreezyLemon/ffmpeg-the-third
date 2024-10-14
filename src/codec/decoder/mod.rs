pub mod decoder;
pub use self::decoder::Decoder;

pub mod video;
pub use self::video::Video;

pub mod audio;
pub use self::audio::Audio;

pub mod subtitle;
pub use self::subtitle::Subtitle;

pub mod slice;

pub mod conceal;
pub use self::conceal::Conceal;

pub mod check;
pub use self::check::Check;

pub mod opened;
pub use self::opened::Opened;

use std::ffi::CString;

use crate::codec::Context;
use crate::codec::Id;
use crate::ffi::*;
use crate::Codec;

pub fn new() -> Decoder {
    Context::new().decoder()
}

pub fn find(id: Id) -> Option<Codec<'static>> {
    unsafe { Codec::from_ptr(avcodec_find_decoder(id.into())) }
}

pub fn find_by_name(name: &str) -> Option<Codec<'static>> {
    unsafe {
        let name = CString::new(name).unwrap();
        Codec::from_ptr(avcodec_find_decoder_by_name(name.as_ptr()))
    }
}
