use crate::codec::codec::*;
use crate::codec::context::*;

pub type Decoder<CodecType, State = Closed> = Context<Decoding, CodecType, State>;

pub type AudioDecoder<State = Closed> = Decoder<AudioType, State>;
pub type VideoDecoder<State = Closed> = Decoder<VideoType, State>;
pub type DataDecoder<State = Closed> = Decoder<DataType, State>;
pub type SubtitleDecoder<State = Closed> = Decoder<SubtitleType, State>;
pub type AttachmentDecoder<State = Closed> = Decoder<AttachmentType, State>;

mod audio;
mod common;
mod opened;
mod subtitle;
mod video;