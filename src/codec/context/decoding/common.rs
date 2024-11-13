use super::{CodecType, Context, Decoding, State};
use crate::Rational;

impl<C: CodecType, S: State> Context<Decoding, C, S> {
    pub fn set_pkt_timebase(&mut self, pkt_timebase: Rational) {
        unsafe {
            (*self.as_mut_ptr()).pkt_timebase = pkt_timebase.into();
        }
    }
}
