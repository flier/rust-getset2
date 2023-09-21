use std::ffi::{CStr, CString};
#[cfg(target_os = "unix")]
use std::{ffi::OsStr, os::unix::ffi::OsStrExt, OsString};

use getset2::Getter;

#[derive(Getter)]
struct Foo<'a> {
    #[get(bytes)]
    str_field: &'a str,

    #[get(bytes)]
    string_field: String,

    #[get(bytes)]
    string_ref_field: &'a String,

    #[get(bytes)]
    cstr_field: &'a CStr,

    #[get(bytes)]
    cstring_field: CString,

    #[get(bytes)]
    vec_field: Vec<u8>,

    #[get(bytes)]
    array_field: [u8; 3],

    #[get(bytes(Foobar::as_bytes))]
    custom_field: Foobar,

    #[cfg(target_os = "unix")]
    #[get(bytes)]
    os_str_field: &'a OsStr,

    #[cfg(target_os = "unix")]
    #[get(bytes)]
    os_string_field: OsString,
}

struct Foobar(Vec<u8>);

impl Foobar {
    fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

#[test]
fn get_bytes() {
    let s = "string".to_owned();
    let foo = Foo {
        str_field: "str",
        string_field: s.clone(),
        string_ref_field: &s,
        cstr_field: CStr::from_bytes_with_nul(b"cstr\0").unwrap(),
        cstring_field: CString::new("cstring").unwrap(),
        vec_field: vec![1, 2, 3],
        array_field: [4, 5, 6],
        custom_field: Foobar(vec![7, 8, 9]),
        #[cfg(target_os = "unix")]
        os_str_field: OsStr::new("os_str"),
        #[cfg(target_os = "unix")]
        os_string_field: OsString::from("os_string"),
    };

    assert_eq!(foo.str_field(), b"str");
    assert_eq!(foo.string_field(), b"string");
    assert_eq!(foo.string_ref_field(), b"string");
    assert_eq!(foo.cstr_field(), b"cstr");
    assert_eq!(foo.cstring_field(), b"cstring");
    assert_eq!(foo.vec_field(), &[1, 2, 3]);
    assert_eq!(foo.array_field(), &[4, 5, 6]);
    assert_eq!(foo.custom_field(), &[7, 8, 9]);
    #[cfg(target_os = "unix")]
    assert_eq!(foo.os_str_field(), b"os_str");
    #[cfg(target_os = "unix")]
    assert_eq!(foo.os_string_field(), b"os_string");
}
