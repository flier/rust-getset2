use std::ffi::{OsStr, OsString};

use getset2::Getter;

#[derive(Default, Getter)]
struct Foo {
    #[get(borrow_mut(str))]
    str_field: String,

    #[get(borrow_mut(OsStr))]
    os_str_field: OsString,

    #[get(borrow(usize), borrow_mut(usize))]
    box_field: Box<usize>,

    #[get(borrow_mut([usize]))]
    vec_field: Vec<usize>,

    #[get(borrow_mut([u8]))]
    array_field: [u8; 4],
}

#[test]
fn get_borrow_mut() {
    let mut foo = Foo {
        str_field: "str".to_owned(),
        os_str_field: OsString::from("foo"),
        box_field: Box::new(123),
        vec_field: vec![1, 2, 3, 4],
        array_field: [1, 2, 3, 4],
    };

    foo.str_field_mut().make_ascii_uppercase();
    foo.os_str_field_mut().make_ascii_uppercase();
    *foo.box_field_mut() = 456;
    foo.vec_field_mut().copy_from_slice(&[5, 6, 7, 8]);
    foo.array_field_mut().copy_from_slice(&[5, 6, 7, 8]);

    assert_eq!(foo.str_field(), "STR");
    assert_eq!(foo.os_str_field(), OsStr::new("FOO"));
    assert_eq!(foo.box_field(), &456);
    assert_eq!(foo.vec_field(), &[5, 6, 7, 8]);
    assert_eq!(foo.array_field(), &[5, 6, 7, 8]);
}
