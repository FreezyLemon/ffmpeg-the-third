use std::ffi::CString;
use std::path::Path;

use crate::macros::{impl_getter_simple, impl_mut_wrapper, impl_owned_wrapper, impl_ref_wrapper};
use crate::util::range::Range;
#[cfg(not(feature = "ffmpeg_5_0"))]
use crate::Codec;
use crate::{ffi::*, Dictionary};
use crate::{format, Error, Packet, Stream};

impl_owned_wrapper!(Input, AVFormatContext);

impl Drop for Input {
    fn drop(&mut self) {
        unsafe {
            avformat_close_input(&mut self.as_mut_ptr());
        }
    }
}

impl_ref_wrapper!(InputRef, AVFormatContext);
impl_mut_wrapper!(InputMut, AVFormatContext);

unsafe impl Send for Input {}

pub struct InputBuilder<'d> {
    path: CString,
    context: Option<Input>,
    force_format: Option<format::Input>,
    options: Option<Dictionary<'d>>,
}

impl<'d> InputBuilder<'d> {
    pub fn context(&mut self, context: Input) -> &mut Self {
        self.context = Some(context);
        self
    }

    pub fn force_format(&mut self, format: format::Input) -> &mut Self {
        self.force_format = Some(format);
        self
    }

    pub fn options(&mut self, options: Dictionary<'d>) -> &mut Self {
        self.options = Some(options);
        self
    }

    pub fn open(self) -> Result<Input, Error> {
        use std::ptr::{null, null_mut};

        let mut ctx = self.context.map_or(null_mut(), |mut ctx| ctx.as_mut_ptr());
        let path = self.path.as_ptr();

        unsafe {
            let fmt = self.force_format.map_or(null(), |fmt| fmt.as_ptr());
            let mut opts = self.options.map_or(null_mut(), |dict| dict.disown());

            let res = avformat_open_input(&mut ctx, path, fmt, &mut opts);

            // TODO: Return dictionary (or contained information) to user somehow
            let _ = Dictionary::own(opts);

            if res != 0 {
                return Err(Error::from(res));
            }

            let res = avformat_find_stream_info(ctx, std::ptr::null_mut());
            if res != 0 {
                avformat_close_input(&mut ctx);
                return Err(Error::from(res));
            }

            Ok(Input::from_ptr(ctx))
        }
    }
}

impl Input {
    pub fn new<'d, P: AsRef<Path>>(path: P) -> InputBuilder<'d> {
        let path = CString::new(path.as_ref().to_str().unwrap()).unwrap();

        InputBuilder {
            path,
            context: None,
            force_format: None,
            options: None,
        }
    }

    pub fn new_prepared<'d, P: AsRef<Path>>(self, path: P) -> InputBuilder<'d> {
        let path = CString::new(path.as_ref().to_str().unwrap()).unwrap();

        InputBuilder {
            path,
            context: Some(self),
            force_format: None,
            options: None,
        }
    }

    unsafe fn from_ptr(ptr: *mut AVFormatContext) -> Self {
        Self(std::ptr::NonNull::new(ptr).unwrap())
    }

    pub fn format(&self) -> format::Input {
        unsafe { format::Input::wrap((*self.as_ptr()).iformat as *mut AVInputFormat) }
    }

    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn video_codec(&self) -> Option<Codec> {
        unsafe {
            let ptr = (*self.as_ptr()).video_codec;

            if ptr.is_null() {
                None
            } else {
                Some(Codec::wrap(ptr))
            }
        }
    }

    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn audio_codec(&self) -> Option<Codec> {
        unsafe {
            let ptr = (*self.as_ptr()).audio_codec;

            if ptr.is_null() {
                None
            } else {
                Some(Codec::wrap(ptr))
            }
        }
    }

    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn subtitle_codec(&self) -> Option<Codec> {
        unsafe {
            let ptr = (*self.as_ptr()).subtitle_codec;

            if ptr.is_null() {
                None
            } else {
                Some(Codec::wrap(ptr))
            }
        }
    }

    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn data_codec(&self) -> Option<Codec> {
        unsafe {
            let ptr = (*self.as_ptr()).data_codec;

            if ptr.is_null() {
                None
            } else {
                Some(Codec::wrap(ptr))
            }
        }
    }

    impl_getter_simple!(probe_score() -> i32; probe_score);

    pub fn packets(&mut self) -> PacketIter {
        PacketIter::new(self)
    }

    pub fn pause(&mut self) -> Result<(), Error> {
        unsafe {
            match av_read_pause(self.as_mut_ptr()) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn play(&mut self) -> Result<(), Error> {
        unsafe {
            match av_read_play(self.as_mut_ptr()) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn seek<R: Range<i64>>(&mut self, ts: i64, range: R) -> Result<(), Error> {
        unsafe {
            match avformat_seek_file(
                self.as_mut_ptr(),
                -1,
                range.start().cloned().unwrap_or(i64::MIN),
                ts,
                range.end().cloned().unwrap_or(i64::MAX),
                0,
            ) {
                s if s >= 0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }
}

pub struct PacketIter<'a> {
    context: &'a mut Input,
}

impl<'a> PacketIter<'a> {
    pub fn new(context: &mut Input) -> PacketIter {
        PacketIter { context }
    }
}

impl<'a> Iterator for PacketIter<'a> {
    type Item = Result<(Stream<'a>, Packet), Error>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let mut packet = Packet::empty();

        match packet.read(self.context) {
            Ok(..) => Some(Ok((
                self.context.stream(packet.stream() as u32).unwrap(),
                packet,
            ))),

            Err(Error::Eof) => None,

            Err(e) => Some(Err(e)),
        }
    }
}

pub fn dump(ctx: &Input, index: i32, url: Option<&str>) {
    let url = url.map(|u| CString::new(u).unwrap());

    unsafe {
        av_dump_format(
            ctx.as_ptr() as *mut _,
            index,
            url.unwrap_or_else(|| CString::new("").unwrap()).as_ptr(),
            0,
        );
    }
}
