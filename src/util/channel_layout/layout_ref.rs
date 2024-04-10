use libc::c_int;

use crate::ffi::*;
use crate::ChannelLayoutInfo;

// #[derive(Clone, Copy)]
#[repr(transparent)]
pub struct ChannelLayoutInfoRef<'a>(&'a AVChannelLayout);

impl<'a> ChannelLayoutInfoRef<'a> {
    pub fn count(&self) -> c_int {
        let a = self.to_owned();
        self.0.nb_channels
    }

    // pub fn to_owned(&self) -> ChannelLayoutInfo {
    //     ChannelLayoutInfo::from(self.0.clone())
    // }
}

impl<'a> From<&'a AVChannelLayout> for ChannelLayoutInfoRef<'a> {
    fn from(value: &'a AVChannelLayout) -> Self {
        Self(value)
    }
}

// impl<'a> ToOwned for ChannelLayoutInfoRef<'a> {
//     type Owned = ChannelLayoutInfo;

//     fn to_owned(&self) -> Self::Owned {
//         todo!()
//     }
// }
