pub mod mask;
pub mod order;

pub use mask::ChannelLayout;
pub use order::ChannelOrder;

#[cfg(feature = "ffmpeg_5_1")]
pub mod channel;
#[cfg(feature = "ffmpeg_5_1")]
pub use channel::Channel;

// TODO:
#[cfg(feature = "ffmpeg_5_1")]
pub mod iter;

#[cfg(feature = "ffmpeg_5_1")]
pub mod layout;
#[cfg(feature = "ffmpeg_5_1")]
pub use layout::ChannelLayoutInfo;
