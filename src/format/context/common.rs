use std::fmt;
use std::mem;
use std::ptr;
use std::rc::Rc;

use super::destructor::{self, Destructor};
use crate::ffi::*;
use crate::{media, Chapter, ChapterMut, DictionaryRef, Stream, StreamMut};
use libc::{c_int, c_uint};

pub struct Context {
    ptr: *mut AVFormatContext,
    dtor: Rc<Destructor>,
}

unsafe impl Send for Context {}

impl Context {
    pub unsafe fn wrap(ptr: *mut AVFormatContext, mode: destructor::Mode) -> Self {
        Context {
            ptr,
            dtor: Rc::new(Destructor::new(ptr, mode)),
        }
    }

    pub unsafe fn as_ptr(&self) -> *const AVFormatContext {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVFormatContext {
        self.ptr
    }

    pub unsafe fn destructor(&self) -> Rc<Destructor> {
        Rc::clone(&self.dtor)
    }
}

impl Context {
    #[inline]
    pub fn nb_streams(&self) -> u32 {
        unsafe { (*self.as_ptr()).nb_streams }
    }

    pub fn stream<'c>(&'c self, index: u32) -> Option<Stream<'c>> {
        if index >= self.nb_streams() {
            return None;
        }

        unsafe {
            let stream_ptr = (*(*self.as_ptr()).streams).add(index as usize);
            Some(Stream::from_ptr(stream_ptr).expect("stream ptr is non-null"))
        }
    }

    pub fn stream_mut<'c>(&'c mut self, index: u32) -> Option<StreamMut<'c>> {
        if index >= self.nb_streams() {
            return None;
        }

        unsafe {
            let stream_ptr = (*(*self.as_ptr()).streams).add(index as usize);
            Some(StreamMut::from_ptr(stream_ptr).expect("stream ptr is non-null"))
        }
    }

    pub fn streams(&self) -> StreamIter {
        StreamIter::new(self)
    }

    pub fn streams_mut(&mut self) -> StreamIterMut {
        StreamIterMut::new(self)
    }

    pub fn bit_rate(&self) -> i64 {
        unsafe { (*self.as_ptr()).bit_rate }
    }

    pub fn duration(&self) -> i64 {
        unsafe { (*self.as_ptr()).duration }
    }

    #[inline]
    pub fn nb_chapters(&self) -> u32 {
        unsafe { (*self.as_ptr()).nb_chapters }
    }

    pub fn chapter<'a, 'b>(&'a self, index: usize) -> Option<Chapter<'b>>
    where
        'a: 'b,
    {
        unsafe {
            if index >= self.nb_chapters() as usize {
                None
            } else {
                Some(Chapter::wrap(self, index))
            }
        }
    }

    pub fn chapter_mut<'a, 'b>(&'a mut self, index: usize) -> Option<ChapterMut<'b>>
    where
        'a: 'b,
    {
        unsafe {
            if index >= self.nb_chapters() as usize {
                None
            } else {
                Some(ChapterMut::wrap(self, index))
            }
        }
    }

    pub fn chapters(&self) -> ChapterIter {
        ChapterIter::new(self)
    }

    pub fn chapters_mut(&mut self) -> ChapterIterMut {
        ChapterIterMut::new(self)
    }

    pub fn metadata(&self) -> DictionaryRef {
        unsafe { DictionaryRef::wrap((*self.as_ptr()).metadata) }
    }
}

pub struct Best<'a> {
    context: &'a Context,

    wanted: i32,
    related: i32,
}

impl<'a> Best<'a> {
    pub unsafe fn new<'b, 'c: 'b>(context: &'c Context) -> Best<'b> {
        Best {
            context,

            wanted: -1,
            related: -1,
        }
    }

    pub fn wanted<'b>(mut self, stream: &'b Stream) -> Best<'a>
    where
        'a: 'b,
    {
        self.wanted = stream.index() as i32;
        self
    }

    pub fn related<'b>(mut self, stream: &'b Stream) -> Best<'a>
    where
        'a: 'b,
    {
        self.related = stream.index() as i32;
        self
    }

    pub fn best(self, kind: media::Type) -> Option<Stream<'a>> {
        unsafe {
            let decoder = ptr::null_mut();
            let index = av_find_best_stream(
                self.context.ptr,
                kind.into(),
                self.wanted as c_int,
                self.related as c_int,
                decoder,
                0,
            );

            match u32::try_from(index) {
                Ok(index) => self.context.stream(index),
                Err(_) => None,
            }
        }
    }
}

pub struct StreamIter<'a> {
    context: &'a Context,
    current: u32,
}

impl<'a> StreamIter<'a> {
    pub fn new<'s, 'c: 's>(context: &'c Context) -> StreamIter<'s> {
        StreamIter {
            context,
            current: 0,
        }
    }
}

impl<'a> StreamIter<'a> {
    pub fn wanted<'b, 'c>(&self, stream: &'b Stream) -> Best<'c>
    where
        'a: 'b,
        'a: 'c,
    {
        unsafe { Best::new(self.context).wanted(stream) }
    }

    pub fn related<'b, 'c>(&self, stream: &'b Stream) -> Best<'c>
    where
        'a: 'b,
        'a: 'c,
    {
        unsafe { Best::new(self.context).related(stream) }
    }

    pub fn best<'b>(&self, kind: media::Type) -> Option<Stream<'b>>
    where
        'a: 'b,
    {
        unsafe { Best::new(self.context).best(kind) }
    }
}

impl<'a> Iterator for StreamIter<'a> {
    type Item = Stream<'a>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        match self.context.stream(self.current) {
            Some(s) => {
                self.current += 1;
                Some(s)
            }
            None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let length = self.context.nb_streams() as usize;

        (
            length - self.current as usize,
            Some(length - self.current as usize),
        )
    }
}

impl<'a> ExactSizeIterator for StreamIter<'a> {}

pub struct StreamIterMut<'a> {
    context: &'a mut Context,
    current: u32,
}

impl<'a> StreamIterMut<'a> {
    pub fn new<'s, 'c: 's>(context: &'c mut Context) -> StreamIterMut<'s> {
        StreamIterMut {
            context,
            current: 0,
        }
    }
}

impl<'a> Iterator for StreamIterMut<'a> {
    type Item = StreamMut<'a>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        match self.context.stream_mut(self.current) {
            Some(s) => {
                self.current += 1;
                Some(s)
            }
            None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let length = self.context.nb_streams() as usize;

        (
            length - self.current as usize,
            Some(length - self.current as usize),
        )
    }
}

impl<'a> ExactSizeIterator for StreamIterMut<'a> {}

pub struct ChapterIter<'a> {
    context: &'a Context,
    current: c_uint,
}

impl<'a> ChapterIter<'a> {
    pub fn new<'s, 'c: 's>(context: &'c Context) -> ChapterIter<'s> {
        ChapterIter {
            context,
            current: 0,
        }
    }
}

impl<'a> Iterator for ChapterIter<'a> {
    type Item = Chapter<'a>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if self.current >= (*self.context.as_ptr()).nb_chapters {
                return None;
            }

            self.current += 1;

            Some(Chapter::wrap(self.context, (self.current - 1) as usize))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        unsafe {
            let length = (*self.context.as_ptr()).nb_chapters as usize;

            (
                length - self.current as usize,
                Some(length - self.current as usize),
            )
        }
    }
}

impl<'a> ExactSizeIterator for ChapterIter<'a> {}

pub struct ChapterIterMut<'a> {
    context: &'a mut Context,
    current: c_uint,
}

impl<'a> ChapterIterMut<'a> {
    pub fn new<'s, 'c: 's>(context: &'c mut Context) -> ChapterIterMut<'s> {
        ChapterIterMut {
            context,
            current: 0,
        }
    }
}

impl<'a> Iterator for ChapterIterMut<'a> {
    type Item = ChapterMut<'a>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if self.current >= (*self.context.as_ptr()).nb_chapters {
                return None;
            }

            self.current += 1;

            Some(ChapterMut::wrap(
                mem::transmute_copy(&self.context),
                (self.current - 1) as usize,
            ))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        unsafe {
            let length = (*self.context.as_ptr()).nb_chapters as usize;

            (
                length - self.current as usize,
                Some(length - self.current as usize),
            )
        }
    }
}

impl<'a> ExactSizeIterator for ChapterIterMut<'a> {}

impl fmt::Debug for Context {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut s = fmt.debug_struct("AVFormatContext");
        s.field("bit_rate", &self.bit_rate());
        s.field("duration", &self.duration());
        s.field("nb_chapters", &self.nb_chapters());
        s.field("nb_streams", &self.nb_streams());
        s.finish()
    }
}
