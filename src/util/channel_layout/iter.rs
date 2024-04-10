use crate::ffi::*;

use super::ChannelLayoutInfo;
use libc::c_void;
use std::ptr::{addr_of_mut, null_mut};

pub struct ChannelLayoutInfoIter {
    opaque: *mut c_void,
}

impl ChannelLayoutInfoIter {
    pub fn new() -> Self {
        Self { opaque: null_mut() }
    }
}

impl Iterator for ChannelLayoutInfoIter {
    type Item = ChannelLayoutInfo<'static>;

    fn next(&mut self) -> Option<Self::Item> {
        // We assume that the returned pointer is valid and the layout has a 'static lifetime
        let layout_ptr = unsafe { av_channel_layout_standard(addr_of_mut!(self.opaque)) };

        match unsafe { layout_ptr.as_ref() } {
            Some(layout_ref) => Some(ChannelLayoutInfo::from(layout_ref)),
            None => None,
        }
    }
}

impl<'a> ChannelLayoutInfo<'a> {
    pub fn all() -> ChannelLayoutInfoIter {
        ChannelLayoutInfoIter::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iter() {
        for layout in ChannelLayoutInfo::all() {
            println!("{layout:?}");
        }
    }
}
