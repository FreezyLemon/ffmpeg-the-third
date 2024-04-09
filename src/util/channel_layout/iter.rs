// use crate::ffi::*;
// use std::ptr::null_mut;

// use libc::c_void;

// pub struct StandardChannelLayoutIter {
//     opaque: *mut c_void,
// }

// impl StandardChannelLayoutIter {
//     pub fn new() -> Self {
//         Self { opaque: null_mut() }
//     }
// }

// impl Iterator for StandardChannelLayoutIter {
//     type Item = &'static ChannelLayoutInfo;

//     fn next(&mut self) -> Option<Self::Item> {
//         let layout_ptr = unsafe { av_channel_layout_standard(addr_of_mut!(self.opaque)) };

//         match unsafe { layout_ptr.as_ref() } {
//             Some(layout_ref) => Some(&ChannelLayoutInfo(*layout_ref)),
//             None => None,
//         }
//     }
// }
