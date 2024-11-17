use super::{Action, CodecType, Context, State};

impl<A: Action, C: CodecType, S: State> Context<A, C, S> {
    pub fn bit_rate(&self) -> i64 {
        unsafe { (*self.as_ptr()).bit_rate }
    }
}
