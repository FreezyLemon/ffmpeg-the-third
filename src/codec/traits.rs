use super::{decoder, encoder};
use crate::codec::{Audio, Id, Video};
use crate::Codec;

pub trait Decoder<'c> {
    fn decoder(self) -> Option<Codec<'c>>;
}

impl<'a> Decoder<'static> for &'a str {
    fn decoder(self) -> Option<Codec<'static>> {
        decoder::find_by_name(self)
    }
}

impl<'c> Decoder<'c> for Id {
    fn decoder(self) -> Option<Codec<'c>> {
        decoder::find(self)
    }
}

impl<'c> Decoder<'c> for Codec<'c> {
    fn decoder(self) -> Option<Codec<'c>> {
        if self.is_decoder() {
            Some(self)
        } else {
            None
        }
    }
}

impl<'c> Decoder<'c> for Option<Codec<'c>> {
    fn decoder(self) -> Option<Codec<'c>> {
        self.and_then(|c| c.decoder())
    }
}

impl<'c> Decoder<'c> for Audio<'c> {
    fn decoder(self) -> Option<Codec<'c>> {
        if self.is_decoder() {
            Some(*self)
        } else {
            None
        }
    }
}

impl<'c> Decoder<'c> for Video<'c> {
    fn decoder(self) -> Option<Codec<'c>> {
        if self.is_decoder() {
            Some(*self)
        } else {
            None
        }
    }
}

pub trait Encoder<'c> {
    fn encoder(self) -> Option<Codec<'c>>;
}

impl<'a> Encoder<'static> for &'a str {
    fn encoder(self) -> Option<Codec<'static>> {
        encoder::find_by_name(self)
    }
}

impl<'c> Encoder<'c> for Id {
    fn encoder(self) -> Option<Codec<'c>> {
        encoder::find(self)
    }
}

impl<'c> Encoder<'c> for Codec<'c> {
    fn encoder(self) -> Option<Codec<'c>> {
        if self.is_encoder() {
            Some(self)
        } else {
            None
        }
    }
}

impl<'c> Encoder<'c> for Option<Codec<'c>> {
    fn encoder(self) -> Option<Codec<'c>> {
        self.and_then(|c| c.encoder())
    }
}

impl<'c> Encoder<'c> for Audio<'c> {
    fn encoder(self) -> Option<Codec<'c>> {
        if self.is_encoder() {
            Some(*self)
        } else {
            None
        }
    }
}

impl<'c> Encoder<'c> for Video<'c> {
    fn encoder(self) -> Option<Codec<'c>> {
        if self.is_encoder() {
            Some(*self)
        } else {
            None
        }
    }
}
