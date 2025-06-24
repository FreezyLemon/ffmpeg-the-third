mod flag;
pub use flag::Flags;

mod immutable;
pub use immutable::DictionaryRef;

mod impls;

mod mutable;
pub use mutable::DictionaryMut;

mod owned;
pub use owned::Dictionary;

mod iter;
pub use iter::Iter;

#[cfg(test)]
mod tests;

#[macro_export]
macro_rules! dict {
    ($($key:expr => $value:expr),* $(,)*) => ({
        let mut dict = ::ffmpeg::Dictionary::new();

        $(
            dict.set($key, $value);
        )*

        dict
    });
}
