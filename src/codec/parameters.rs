use crate::macros::impl_getter_into;
use crate::macros::{impl_for_many, impl_mut_wrapper, impl_owned_wrapper, impl_ref_wrapper};

use super::{Context, Id};
use crate::ffi::*;
use crate::media;

unsafe impl Send for Parameters {}

impl_owned_wrapper!(
    Parameters,
    AVCodecParameters,
    avcodec_parameters_alloc,
    freep avcodec_parameters_free
);

impl Clone for Parameters {
    fn clone(&self) -> Self {
        let mut res = Self::new();
        res.clone_from(self);

        res
    }

    fn clone_from(&mut self, source: &Self) {
        unsafe {
            avcodec_parameters_copy(self.as_mut_ptr(), source.as_ptr());
        }
    }
}

impl_ref_wrapper!(ParametersRef, AVCodecParameters);
impl_mut_wrapper!(ParametersMut, AVCodecParameters);

impl_for_many! {
    impl for Parameters, ParametersRef<'p>, ParametersMut<'p> {
        impl_getter_into!(medium() -> media::Type; codec_type);
        impl_getter_into!(id() -> Id; codec_id);
    }
}

impl Default for Parameters {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: AsRef<Context>> From<C> for Parameters {
    fn from(context: C) -> Parameters {
        let mut parameters = Parameters::new();
        let context = context.as_ref();
        unsafe {
            avcodec_parameters_from_context(parameters.as_mut_ptr(), context.as_ptr());
        }
        parameters
    }
}
