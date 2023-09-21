use std::{
    collections::HashMap,
    ffi::{CStr, CString, OsStr, OsString},
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};

use getset2::Getter;

mod foo {
    use getset2::Getter;

    #[derive(Default, Getter)]
    #[get(copy)]
    pub struct Bar {
        private_field: usize,

        pub public_field: usize,

        #[get(pub)]
        pub_field: usize,

        #[get(pub(self))]
        pub_self_field: usize,

        #[get(pub(crate))]
        pub_crate_field: usize,

        #[get(pub(super))]
        pub_super_field: usize,

        #[get(pub(in crate::foo))]
        pub_in_module_field: usize,
    }

    #[test]
    fn test_private() {
        let bar = Bar::default();

        assert_eq!(bar.private_field(), 0);
        assert_eq!(bar.pub_self_field(), 0);
    }

    #[test]
    fn test_get_pub() {
        let bar = Bar::default();

        assert_eq!(bar.pub_in_module_field(), 0);
    }
}

#[test]
fn test_public() {
    let foobar = foo::Bar::default();

    assert_eq!(foobar.public_field(), 0);
}

#[test]
fn test_get_pub() {
    let foobar = foo::Bar::default();

    assert_eq!(foobar.pub_field(), 0);
    assert_eq!(foobar.pub_crate_field(), 0);
    assert_eq!(foobar.pub_super_field(), 0);
}

#[test]
fn test_get_mut() {
    #[derive(Default, Getter)]
    struct Foo {
        #[get(mut, copy)]
        field: usize,
    }

    let mut foo = Foo::default();

    *foo.field_mut() = 1;

    assert_eq!(foo.field(), 1);
}

#[test]
fn test_prefix() {
    #[derive(Default, Getter)]
    struct Foo {
        #[get(prefix = "with", copy, mut)]
        pub prefix_field: usize,
    }

    let mut foo = Foo::default();

    *foo.with_prefix_field_mut() = 1;

    assert_eq!(foo.with_prefix_field(), 1);
}

#[test]
fn test_suffix() {
    #[derive(Default, Getter)]
    struct Foo {
        #[get(suffix = "num", copy, mut)]
        pub suffix_field: usize,
    }

    let mut foo = Foo::default();

    *foo.suffix_field_num_mut() = 1;

    assert_eq!(foo.suffix_field_num(), 1);
}

#[test]
fn test_get_copy() {
    #[derive(Getter)]
    struct Foo<'a> {
        #[get(copy, mut)]
        copy_field: usize,

        #[get(copy, mut)]
        copy_ref_field: &'a usize,
    }

    let mut foo = Foo {
        copy_field: 0,
        copy_ref_field: &0,
    };

    *foo.copy_field_mut() = 1;
    assert_eq!(foo.copy_field(), 1);

    *foo.copy_ref_field_mut() = &2;
    assert_eq!(foo.copy_ref_field(), 2);
}

#[test]
fn test_get_clone() {
    #[derive(Getter)]
    struct Foo<'a> {
        #[get(clone)]
        pub clone_field: PathBuf,

        #[get(clone)]
        pub clone_ref_field: &'a PathBuf,
    }

    let p = PathBuf::from("/tmp");
    let foo = Foo {
        clone_field: PathBuf::from("/tmp"),
        clone_ref_field: &p,
    };

    assert_eq!(foo.clone_field(), PathBuf::from("/tmp"));
    assert_eq!(foo.clone_ref_field(), Path::new("/tmp"));
}

#[test]
fn test_get_opt() {
    #[derive(Default, Getter)]
    struct Foo {
        #[get(opt, mut)]
        pub option_field: Option<HashMap<String, usize>>,
    }

    let mut foo = Foo::default();

    let _ = foo.option_field.insert(HashMap::new());

    foo.option_field_mut()
        .unwrap()
        .insert("foo".to_owned(), 123);

    assert_eq!(foo.option_field().unwrap().get("foo"), Some(&123));
}

#[test]
fn test_get_slice() {
    #[derive(Default, Getter)]
    struct Foo {
        #[get(slice, mut_slice)]
        pub vec_field: Vec<u8>,

        #[get(slice, mut_slice)]
        pub array_field: [u8; 4],
    }

    let mut foo = Foo::default();

    foo.vec_field = vec![0; 3];
    foo.vec_field_mut().copy_from_slice(&[1, 2, 3]);

    foo.array_field_mut()
        .copy_from_slice(0x12345678u32.to_le_bytes().as_slice());

    assert_eq!(foo.vec_field(), &[1, 2, 3]);
    assert_eq!(foo.array_field(), &[120, 86, 52, 18]);
}

#[test]
fn test_get_str() {
    #[derive(Default, Getter)]
    struct Foo {
        #[get(str, mut_str)]
        string_field: String,
    }

    let mut foo = Foo::default();

    foo.string_field = "foo".to_owned();

    foo.string_field_mut().make_ascii_uppercase();

    assert_eq!(foo.string_field(), "FOO");
}

#[test]
fn test_get_bytes() {
    #[derive(Default, Getter)]
    struct Foo<'a> {
        #[get(bytes)]
        str_field: &'a str,

        #[get(bytes)]
        string_field: String,

        #[get(bytes(CStr::to_bytes))]
        cstr_field: &'a CStr,

        #[get(bytes)]
        cstring_field: CString,
    }

    let foo = Foo {
        str_field: "str",
        string_field: "string".to_owned(),
        cstr_field: CStr::from_bytes_with_nul(b"cstr\0").unwrap(),
        cstring_field: CString::new("cstring").unwrap(),
    };

    assert_eq!(foo.str_field(), b"str");
    assert_eq!(foo.string_field(), b"string");
    assert_eq!(foo.cstr_field(), b"cstr");
    assert_eq!(foo.cstring_field(), b"cstring");
}

#[test]
fn test_get_borrow() {
    #[derive(Default, Getter)]
    struct Foo {
        #[get(borrow(str))]
        str_field: String,

        #[get(borrow(CStr))]
        cstr_field: CString,

        #[get(borrow(OsStr))]
        os_str_field: OsString,

        #[get(borrow(Path))]
        path_field: PathBuf,

        #[get(borrow(usize))]
        rc_field: Rc<usize>,

        #[get(borrow(usize))]
        arc_field: Arc<usize>,

        #[get(borrow(usize))]
        box_field: Box<usize>,

        #[get(borrow([usize]))]
        vec_field: Vec<usize>,

        #[get(borrow([u8]))]
        array_field: [u8; 4],
    }

    let foo = Foo {
        str_field: "str".to_owned(),
        cstr_field: CString::new("cstr").unwrap(),
        os_str_field: OsString::from("os_str"),
        path_field: PathBuf::from("/tmp"),
        rc_field: Rc::new(123),
        arc_field: Arc::new(456),
        box_field: Box::new(789),
        vec_field: vec![1, 2, 3, 4],
        array_field: [1, 2, 3, 4],
    };

    assert_eq!(foo.str_field(), "str");
    assert_eq!(
        foo.cstr_field(),
        CStr::from_bytes_with_nul(b"cstr\0").unwrap()
    );
    assert_eq!(foo.os_str_field(), OsStr::new("os_str"));
    assert_eq!(foo.path_field(), Path::new("/tmp"));
    assert_eq!(foo.rc_field(), &123);
    assert_eq!(foo.arc_field(), &456);
    assert_eq!(foo.box_field(), &789);
    assert_eq!(foo.vec_field(), &[1, 2, 3, 4]);
    assert_eq!(foo.array_field(), &[1, 2, 3, 4]);
}

#[test]
fn test_get_borrow_mut() {
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

#[test]
fn test_const_getter() {
    #[derive(Default, Getter)]
    #[get(pub, const, mut)]
    struct Foo {
        private_field: usize,

        #[get(copy)]
        copy_field: usize,

        #[get(clone, const(false))]
        clone_field: usize,
    }

    let mut foo = Foo::default();

    *foo.private_field_mut() = 123;
    assert_eq!(foo.private_field(), &123);

    *foo.copy_field_mut() = 456;
    assert_eq!(foo.copy_field(), 456);

    *foo.clone_field_mut() = 789;
    assert_eq!(foo.clone_field(), 789);
}

#[test]
fn test_unnamed_struct() {
    #[derive(Default, Getter)]
    #[get(pub, copy)]
    pub struct Unnamed(#[get(rename(x))] usize, usize);

    let unnamed = Unnamed(123, 456);

    assert_eq!(unnamed.x(), 123);
    assert_eq!(unnamed.arg1(), 456);
}

#[test]
fn test_get_attr() {
    #[derive(Default, Getter)]
    #[get(pub, const, copy)]
    struct Foo {
        #[doc = "test"]
        #[get(attr(rustfmt::skip))]
        #[get(attr(clippy::cyclomatic_complexity = "100"))]
        bar: usize,
    }

    let foo = Foo { bar: 123 };

    assert_eq!(foo.bar(), 123);
}

#[test]
fn test_get_attrs() {
    #[derive(Default, Getter)]
    #[get(pub, const, copy, attrs("rustfmt", "clippy"))]
    struct Foo {
        #[doc = "test"]
        #[rustfmt::skip]
        #[clippy::cyclomatic_complexity = "100"]
        bar: usize,
    }

    let foo = Foo { bar: 123 };

    assert_eq!(foo.bar(), 123);
}
