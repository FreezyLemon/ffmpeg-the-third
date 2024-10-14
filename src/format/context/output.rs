use std::ffi::CString;
use std::mem::size_of;
use std::path::Path;
use std::ptr;

use crate::codec::traits;
use crate::ffi::*;
use crate::macros::impl_owned_wrapper;
use crate::{format, ChapterMut, Dictionary, Error, Rational, StreamMut};

impl_owned_wrapper!(Output, AVFormatContext);

unsafe impl Send for Output {}

pub struct OutputBuilder<'d> {
    path: CString,
    context: Option<Output>,
    force_format: Option<format::Output>,
    options: Option<Dictionary<'d>>,
}

impl<'d> OutputBuilder<'d> {
    pub fn context(&mut self, context: Output) -> &mut Self {
        self.context = Some(context);
        self
    }

    pub fn force_format(&mut self, format: format::Output) -> &mut Self {
        self.force_format = Some(format);
        self
    }

    pub fn options(&mut self, options: Dictionary<'d>) -> &mut Self {
        self.options = Some(options);
        self
    }

    pub fn open(self) -> Result<Output, Error> {
        use std::ptr::{null, null_mut};

        let mut ctx = self.context.map_or(null_mut(), |mut ctx| ctx.as_mut_ptr());
        let path = self.path.as_ptr();

        unsafe {
            let fmt = self.force_format.map_or(null(), |fmt| fmt.as_ptr());

            // FIXME: Check how avformat_alloc_output_context2 and avio_open2 use these params
            //        and check if the API can be improved

            let res = avformat_alloc_output_context2(&mut ctx, fmt, ptr::null_mut(), path);

            if res != 0 {
                return Err(Error::from(res));
            }

            let mut opts = self.options.map_or(null_mut(), |dict| dict.disown());
            let res = avio_open2(&mut (*ctx).pb, path, AVIO_FLAG_WRITE, null(), &mut opts);

            // TODO: Return dictionary (or contained information) to user somehow
            let _ = Dictionary::own(opts);

            if res >= 0 {
                Ok(Output::from_ptr(ctx))
            } else {
                Err(Error::from(res))
            }
        }
    }
}

impl Output {
    pub fn new<'d, P: AsRef<Path>>(path: P) -> OutputBuilder<'d> {
        let path = CString::new(path.as_ref().to_str().unwrap()).unwrap();

        OutputBuilder {
            path,
            context: None,
            force_format: None,
            options: None,
        }
    }

    unsafe fn from_ptr(ptr: *mut AVFormatContext) -> Self {
        Self(std::ptr::NonNull::new(ptr).unwrap())
    }

    pub fn format(&self) -> format::Output {
        unsafe { format::Output::wrap((*self.as_ptr()).oformat as *mut AVOutputFormat) }
    }

    pub fn write_header(&mut self) -> Result<(), Error> {
        unsafe {
            match avformat_write_header(self.as_mut_ptr(), ptr::null_mut()) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn write_header_with(&mut self, options: Dictionary) -> Result<Dictionary, Error> {
        unsafe {
            let mut opts = options.disown();
            let res = avformat_write_header(self.as_mut_ptr(), &mut opts);

            match res {
                0 => Ok(Dictionary::own(opts)),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn write_trailer(&mut self) -> Result<(), Error> {
        unsafe {
            match av_write_trailer(self.as_mut_ptr()) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn add_stream<'c, E: traits::Encoder<'c>>(&mut self, codec: E) -> Result<StreamMut, Error> {
        unsafe {
            let codec = codec.encoder();
            let codec = codec.map_or(ptr::null(), |c| c.as_ptr());
            let ptr = avformat_new_stream(self.as_mut_ptr(), codec);

            if ptr.is_null() {
                return Err(Error::Unknown);
            }

            let index = (*self.ctx.as_ptr()).nb_streams - 1;

            Ok(self.stream_mut(index).unwrap())
        }
    }

    pub fn add_chapter<R: Into<Rational>, S: AsRef<str>>(
        &mut self,
        id: i64,
        time_base: R,
        start: i64,
        end: i64,
        title: S,
    ) -> Result<ChapterMut, Error> {
        // avpriv_new_chapter is private (libavformat/internal.h)

        if start > end {
            return Err(Error::InvalidData);
        }

        let mut existing = None;
        for chapter in self.chapters() {
            if chapter.id() == id {
                existing = Some(chapter.index());
                break;
            }
        }

        let index = match existing {
            Some(index) => index,
            None => unsafe {
                let ptr = av_mallocz(size_of::<AVChapter>())
                    .as_mut()
                    .ok_or(Error::Bug)?;
                let mut nb_chapters = (*self.as_ptr()).nb_chapters as i32;

                // chapters array will be freed by `avformat_free_context`
                av_dynarray_add(
                    &mut (*self.as_mut_ptr()).chapters as *mut _ as *mut libc::c_void,
                    &mut nb_chapters,
                    ptr,
                );

                if nb_chapters > 0 {
                    (*self.as_mut_ptr()).nb_chapters = nb_chapters as u32;
                    let index = (*self.ctx.as_ptr()).nb_chapters - 1;
                    index as usize
                } else {
                    // failed to add the chapter
                    av_freep(ptr);
                    return Err(Error::Bug);
                }
            },
        };

        let mut chapter = self.chapter_mut(index).ok_or(Error::Bug)?;

        chapter.set_id(id);
        chapter.set_time_base(time_base);
        chapter.set_start(start);
        chapter.set_end(end);
        chapter.set_metadata("title", title);

        Ok(chapter)
    }

    pub fn set_metadata(&mut self, dictionary: Dictionary) {
        unsafe {
            (*self.as_mut_ptr()).metadata = dictionary.disown();
        }
    }
}

pub fn dump(ctx: &Output, index: i32, url: Option<&str>) {
    let url = url.map(|u| CString::new(u).unwrap());

    unsafe {
        av_dump_format(
            ctx.as_ptr() as *mut _,
            index,
            url.unwrap_or_else(|| CString::new("").unwrap()).as_ptr(),
            1,
        );
    }
}
