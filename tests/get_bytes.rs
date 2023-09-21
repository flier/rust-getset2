use std::ffi::{CStr, CString};

use getset2::Getter;

#[derive(Getter)]
struct Foo<'a> {
    #[get(bytes)]
    str_field: &'a str,

    #[get(bytes)]
    string_field: String,

    #[get(bytes)]
    string_ref_field: &'a String,

    #[get(bytes(CStr::to_bytes))]
    cstr_field: &'a CStr,

    #[get(bytes(CString::as_bytes))]
    cstring_field: CString,
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
    };

    assert_eq!(foo.str_field(), b"str");
    assert_eq!(foo.string_field(), b"string");
    assert_eq!(foo.string_ref_field(), b"string");
    assert_eq!(foo.cstr_field(), b"cstr");
    assert_eq!(foo.cstring_field(), b"cstring");
}
