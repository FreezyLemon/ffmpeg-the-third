use crate::ffi::*;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Engine {
    Software,
    SoundExchange,
}

impl From<SwrEngine> for Engine {
    fn from(value: SwrEngine) -> Engine {
        use SwrEngine as AV;

        match value {
            AV::SWR_ENGINE_SWR => Engine::Software,
            AV::SWR_ENGINE_SOXR => Engine::SoundExchange,
            AV::SWR_ENGINE_NB => Engine::Software,

            #[cfg(feature = "non-exhaustive-enums")]
            _ => unimplemented!(),
        }
    }
}

impl From<Engine> for SwrEngine {
    fn from(value: Engine) -> SwrEngine {
        match value {
            Engine::Software => Self::SWR_ENGINE_SWR,
            Engine::SoundExchange => Self::SWR_ENGINE_SOXR,
        }
    }
}
