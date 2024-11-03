use crate::ffi::*;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Filter {
    Cubic,
    BlackmanNuttall,
    Kaiser,
}

impl From<SwrFilterType> for Filter {
    fn from(value: SwrFilterType) -> Filter {
        use SwrFilterType as AV;

        match value {
            AV::SWR_FILTER_TYPE_CUBIC => Filter::Cubic,
            AV::SWR_FILTER_TYPE_BLACKMAN_NUTTALL => Filter::BlackmanNuttall,
            AV::SWR_FILTER_TYPE_KAISER => Filter::Kaiser,

            #[cfg(feature = "non-exhaustive-enums")]
            _ => unimplemented!(),
        }
    }
}

impl From<Filter> for SwrFilterType {
    fn from(value: Filter) -> SwrFilterType {
        match value {
            Filter::Cubic => Self::SWR_FILTER_TYPE_CUBIC,
            Filter::BlackmanNuttall => Self::SWR_FILTER_TYPE_BLACKMAN_NUTTALL,
            Filter::Kaiser => Self::SWR_FILTER_TYPE_KAISER,
        }
    }
}
