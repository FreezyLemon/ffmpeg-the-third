//! Module containing wrappers for AVDictionary.
// TODO: Add an example here after the codec rework, where a dictionary is used
//       to pass information to avcodec_open2.

mod flag;
pub use flag::Flags;

mod impls;

mod borrowed;
pub use self::borrowed::DictionaryRef;

mod borrowed_mut;
pub use self::borrowed_mut::DictionaryMut;

mod owned;
pub use self::owned::Dictionary;

mod iter;
pub use self::iter::Iter;

#[cfg(test)]
mod tests;

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
