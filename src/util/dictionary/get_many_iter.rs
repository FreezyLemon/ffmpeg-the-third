use std::ffi::CString;

use libc::c_int;

use super::{Dictionary, DictionaryEntry};

pub struct DictionaryGetManyIter<'d: 'e, 'e>
{
    dict: Dictionary<'d>,
    key: CString,
    flags: c_int,
    prev: Option<DictionaryEntry<'e>>,
}

impl<'d: 'e, 'e> DictionaryGetManyIter<'d, 'e> {
    pub fn new(dict: Dictionary<'d>, key: CString, flags: c_int) -> Self {
        Self {
            dict,
            key,
            flags,
            prev: None,
        }
    }
}

impl<'d: 'e, 'e> Iterator for DictionaryGetManyIter<'d, 'e> {
    type Item = DictionaryEntry<'e>;

    fn next(&mut self) -> Option<Self::Item>
    {
        match self.dict.get(&self.key, self.prev, self.flags) {
            Ok(entry) => {
                self.prev = Some(entry);
                Some(entry)
            }
            Err(_) => None,
        }
    }
}
