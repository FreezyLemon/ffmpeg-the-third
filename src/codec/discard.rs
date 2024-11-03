use crate::ffi::*;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Discard {
    None,
    Default,
    NonReference,
    Bidirectional,
    NonIntra,
    NonKey,
    All,
}

impl From<AVDiscard> for Discard {
    fn from(value: AVDiscard) -> Self {
        use AVDiscard as AV;

        match value {
            AV::AVDISCARD_NONE => Discard::None,
            AV::AVDISCARD_DEFAULT => Discard::Default,
            AV::AVDISCARD_NONREF => Discard::NonReference,
            AV::AVDISCARD_BIDIR => Discard::Bidirectional,
            AV::AVDISCARD_NONINTRA => Discard::NonIntra,
            AV::AVDISCARD_NONKEY => Discard::NonKey,
            AV::AVDISCARD_ALL => Discard::All,

            #[cfg(feature = "non-exhaustive-enums")]
            _ => unimplemented!(),
        }
    }
}

impl From<Discard> for AVDiscard {
    fn from(value: Discard) -> AVDiscard {
        match value {
            Discard::None => Self::AVDISCARD_NONE,
            Discard::Default => Self::AVDISCARD_DEFAULT,
            Discard::NonReference => Self::AVDISCARD_NONREF,
            Discard::Bidirectional => Self::AVDISCARD_BIDIR,
            Discard::NonIntra => Self::AVDISCARD_NONINTRA,
            Discard::NonKey => Self::AVDISCARD_NONKEY,
            Discard::All => Self::AVDISCARD_ALL,
        }
    }
}
