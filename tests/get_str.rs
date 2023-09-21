use std::ffi::CStr;

use getset2::Getter;

#[derive(Getter)]
struct Foo<'a> {
    #[get(str, mut_str)]
    string_field: String,

    #[get(str, mut_str)]
    string_ref_field: &'a mut String,

    #[get(str(cstr_to_str))]
    cstr_field: &'a CStr,
}

fn cstr_to_str(s: &CStr) -> &str {
    s.to_str().unwrap()
}

#[test]
fn get_str() {
    let mut s = "bar".to_string();
    let mut foo = Foo {
        string_field: "foo".to_string(),
        string_ref_field: &mut s,
        cstr_field: CStr::from_bytes_with_nul(b"foo\0").unwrap(),
    };

    foo.string_field_mut().make_ascii_uppercase();
    foo.string_ref_field_mut().make_ascii_uppercase();

    assert_eq!(foo.string_field(), "FOO");
    assert_eq!(foo.string_ref_field(), "BAR");
    assert_eq!(foo.cstr_field(), "foo");
}
