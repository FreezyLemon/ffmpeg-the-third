pub mod channel;
pub mod iter;
pub mod mask;
pub mod order;

pub use channel::Channel;
pub use mask::ChannelLayout;
pub use order::ChannelOrder;

#[cfg(feature = "ffmpeg_5_1")]
pub mod layout;
#[cfg(feature = "ffmpeg_5_1")]
pub use layout::ChannelLayoutInfo;
