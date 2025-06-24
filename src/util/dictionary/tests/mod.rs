use super::*;
use crate::ffi::*;

// `cargo test` does not have support for "fails to compile" tests, so we use trybuild
#[test]
fn test_failing() {
    let t = trybuild::TestCases::new();
    t.compile_fail("src/util/dictionary/tests/fail.rs");
}

#[test]
fn simple() {
    let mut d = Dictionary::new();
    assert!(d.is_empty());
    assert_eq!(d.len(), 0);

    d.set("test", "42");
    d.set("foo", "100");
    d.set("bar", "Hello World!");

    assert!(!d.is_empty());
    assert_eq!(d.len(), 3);
    assert_eq!(d.get("foo"), Some("100"));
    assert_eq!(d.get("test"), Some("42"));
    assert_eq!(d.get("bar"), Some("Hello World!"));
    assert_eq!(d.get("baz"), None);
}

#[test]
fn with_flags() {
    let mut d = Dictionary::new();

    d.set("test", "42");

    assert_eq!(d.get_with_flags("TEST", Flags::MATCH_CASE), None);

    assert_eq!(d.get("test"), Some("42"));

    d.set_with_flags("TEST", "100", Flags::MATCH_CASE);
    assert_eq!(d.get_with_flags("TEST", Flags::MATCH_CASE), Some("100"));
    assert_eq!(d.get_with_flags("test", Flags::MATCH_CASE), Some("42"));

    d.unset_with_flags("test", Flags::MATCH_CASE);
    assert_eq!(d.get_with_flags("test", Flags::MATCH_CASE), None);
    assert_eq!(d.get_with_flags("TEST", Flags::MATCH_CASE), Some("100"));
    assert_eq!(d.get("test"), Some("100"));

    d.set_with_flags("foo", "123", Flags::DONT_OVERWRITE);
    assert_eq!(d.get("foo"), Some("123"));

    d.set_with_flags("test", "321", Flags::DONT_OVERWRITE);
    assert_eq!(d.get("test"), Some("100"));

    d.set_with_flags("TEST", "Hello", Flags::MATCH_CASE);
    d.set_with_flags("TEST", ", World!", Flags::MATCH_CASE | Flags::APPEND);
    assert_eq!(d.get("TEST"), Some("Hello, World!"));

    assert_eq!(
        d.get_with_flags("te", Flags::IGNORE_SUFFIX),
        Some("Hello, World!")
    );
}

#[test]
fn owned_and_ref() {
    let mut d = Dictionary::new();
    d.set("foo", "Hello, World!");
    d.set("bar", "123");
    d.set("testing", "321");

    let r = d.as_ref();

    // This would cause an error:
    // d.set("baz", "impossible");
    // let m = d.as_mut();

    assert_eq!(r.get("bar"), Some("123"));
}

// TODO: add multikey tests

#[test]
fn mut_inside_struct() {
    struct ContainsDictionary {
        x: i32,
        dict: *mut AVDictionary,
        y: f32,
    }

    let mut t = ContainsDictionary {
        x: 42,
        dict: std::ptr::null_mut(),
        y: -17.0,
    };

    let mut dict = unsafe { DictionaryMut::from_raw(&mut t.dict) };

    t.x = 100; // only t.dict is (mutably) borrowed

    // This would cause an error:
    // t.dict = std::ptr::null_mut();

    dict.set("testing", "123");
    #[allow(clippy::drop_non_drop)] // here for clarity
    drop(dict);

    assert!(!t.dict.is_null());
    let dict = unsafe { DictionaryRef::from_raw(t.dict) };

    assert_eq!(dict.get("testing"), Some("123"));
    assert_eq!(dict.len(), 1);

    assert_eq!(t.x, 100);
    assert_eq!(t.y, -17.0);
}
