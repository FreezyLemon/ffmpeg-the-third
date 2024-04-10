pub mod mask;
pub use mask::ChannelLayout;

#[cfg(feature = "ffmpeg_5_1")]
pub mod channel;
#[cfg(feature = "ffmpeg_5_1")]
pub use channel::Channel;

#[cfg(feature = "ffmpeg_5_1")]
pub mod order;
#[cfg(feature = "ffmpeg_5_1")]
pub use order::ChannelOrder;

#[cfg(feature = "ffmpeg_5_1")]
pub mod iter;

#[cfg(feature = "ffmpeg_5_1")]
pub mod layout;
#[cfg(feature = "ffmpeg_5_1")]
pub use layout::ChannelLayoutInfo;
