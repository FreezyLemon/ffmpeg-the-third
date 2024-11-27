use crate::ffi::*;
use libc::c_int;

bitflags::bitflags! {
    pub struct Flags: c_int {
        /// Only get an entry with exact-case key match.
        const MATCH_CASE        = AV_DICT_MATCH_CASE;

        /// Return first entry in a dictionary whose first part corresponds to the
        /// search key, ignoring the suffix of the found key string.
        const IGNORE_SUFFIX     = AV_DICT_IGNORE_SUFFIX;

        /// Take ownership of a key that's been allocated with av_malloc() or
        /// another memory allocation function.
        const DONT_STRDUP_KEY   = AV_DICT_DONT_STRDUP_KEY;

        /// Take ownership of a value that's been allocated with av_malloc()
        /// or another memory allocation function.
        const DONT_STRDUP_VAL   = AV_DICT_DONT_STRDUP_VAL;

        /// Don't overwrite existing entries.
        const DONT_OVERWRITE    = AV_DICT_DONT_OVERWRITE;

        /// If the entry already exists, append to it.
        ///
        /// Note that no delimiter is added, the strings are simply concatenated.
        const APPEND            = AV_DICT_APPEND;

        /// Allow to store several equal keys in the dictionary.
        const MULTIKEY          = AV_DICT_MULTIKEY;
    }
}
