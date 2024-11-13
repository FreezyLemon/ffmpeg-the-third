use crate::codec::codec::*;
use crate::codec::context::*;

pub type Encoder<CodecType, State = Closed> = Context<Encoding, CodecType, State>;

pub type AudioEncoder<State = Closed> = Encoder<AudioType, State>;
pub type VideoEncoder<State = Closed> = Encoder<VideoType, State>;
pub type DataEncoder<State = Closed> = Encoder<DataType, State>;
pub type SubtitleEncoder<State = Closed> = Encoder<SubtitleType, State>;
pub type AttachmentEncoder<State = Closed> = Encoder<AttachmentType, State>;

mod audio;
mod common;
mod video;
