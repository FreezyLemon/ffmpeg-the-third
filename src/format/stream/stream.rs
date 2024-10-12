use super::Disposition;
use crate::codec;
use crate::codec::ParametersRef;
use crate::ffi::*;
use crate::macros::{
    impl_for_many, impl_getter_into, impl_getter_simple, impl_mut_wrapper, impl_ref_wrapper,
    impl_setter_into,
};
use crate::packet::side_data::SideDataIter;
use crate::Dictionary;
use crate::{DictionaryRef, Discard, Rational};

impl_ref_wrapper!(Stream, AVStream);
impl_mut_wrapper!(StreamMut, AVStream);

impl_for_many! {
    impl for Stream<'s>, StreamMut<'s> {
        impl_getter_simple!(id() -> i32; id);
        impl_getter_simple!(index() -> usize; index);

        #[cfg(not(feature = "ffmpeg_5_0"))]
        pub fn codec(&self) -> codec::Context {
            unsafe { codec::Context::wrap((*self.as_ptr()).codec, Some(self.context.destructor())) }
        }

        pub fn parameters(&self) -> ParametersRef {
            unsafe { ParametersRef::from_ptr((*self.as_ptr()).codecpar).expect("ptr is nonnull") }
        }

        impl_getter_into!(time_base() -> Rational; time_base);
        impl_getter_simple!(start_time() -> i64; start_time);
        impl_getter_simple!(duration() -> i64; duration);
        impl_getter_simple!(frames() -> i64; nb_frames);

        pub fn side_data(&self) -> SideDataIter {
            // SAFETY:
            // - Pointer is assumed to be valid (FFmpeg API)
            // - Lifetime of iterator is 's (lifetime of stream), so it the pointer can't
            //   outlive the containing AVStream object
            unsafe {
                SideDataIter::new(
                    (*self.as_ptr()).side_data,
                    (*self.as_ptr()).nb_side_data,
                )
            }
        }

        pub fn disposition(&self) -> Disposition {
            unsafe { Disposition::from_bits_truncate((*self.as_ptr()).disposition) }
        }

        impl_getter_into!(discard() -> Discard; discard);
        impl_getter_into!(avg_frame_rate() -> Rational; avg_frame_rate);

        pub fn metadata(&self) -> DictionaryRef {
            unsafe { DictionaryRef::wrap((*self.as_ptr()).metadata) }
        }
    }
}

impl<'s> StreamMut<'s> {
    impl_setter_into!(set_time_base(Rational); time_base);
    impl_setter_into!(set_rate(Rational); r_frame_rate);
    impl_setter_into!(set_avg_frame_rate(Rational); avg_frame_rate);

    pub fn set_parameters<P: Into<codec::Parameters>>(&mut self, parameters: P) {
        let parameters = parameters.into();

        unsafe {
            avcodec_parameters_copy((*self.as_mut_ptr()).codecpar, parameters.as_ptr());
        }
    }

    pub fn set_metadata(&mut self, metadata: Dictionary) {
        unsafe {
            let metadata = metadata.disown();
            (*self.as_mut_ptr()).metadata = metadata;
        }
    }
}
