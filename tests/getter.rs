use std::{collections::HashMap, path::PathBuf};

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
    fn test_private_getter() {
        let bar = Bar::default();

        assert_eq!(bar.private_field(), 0);
        assert_eq!(bar.pub_self_field(), 0);
    }

    #[test]
    fn test_pub_getter() {
        let bar = Bar::default();

        assert_eq!(bar.pub_in_module_field(), 0);
    }
}

#[test]
fn test_public_getter() {
    let foobar = foo::Bar::default();

    assert_eq!(foobar.public_field(), 0);
}

#[test]
fn test_pub_getter() {
    let foobar = foo::Bar::default();

    assert_eq!(foobar.pub_field(), 0);
    assert_eq!(foobar.pub_crate_field(), 0);
    assert_eq!(foobar.pub_super_field(), 0);
}

#[test]
fn test_mut_getter() {
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
fn test_getter_with_prefix() {
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
fn test_getter_with_suffix() {
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
fn test_copy_field() {
    #[derive(Default, Getter)]
    struct Foo {
        #[get(copy, mut)]
        pub copy_field: usize,
    }

    let mut foo = Foo::default();

    *foo.copy_field_mut() = 1;

    let p = foo.copy_field();

    *foo.copy_field_mut() = 2;

    assert_eq!(p, 1);
    assert_eq!(foo.copy_field(), 2);
}

#[test]
fn test_clone_field() {
    #[derive(Default, Getter)]
    struct Foo {
        #[get(clone, mut)]
        pub clone_field: PathBuf,
    }

    let mut foo = Foo::default();

    foo.clone_field_mut().push("/tmp");

    let p = foo.clone_field();

    foo.clone_field_mut().push("clone");

    assert_eq!(p, PathBuf::from("/tmp"));
    assert_eq!(foo.clone_field(), PathBuf::from("/tmp/clone"));
}

#[test]
fn test_opt_field() {
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
fn test_slice_field() {
    #[derive(Default, Getter)]
    struct Foo {
        #[get(slice, mut)]
        pub vec_field: Vec<u8>,

        #[get(slice, mut)]
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
fn test_string_field() {
    #[derive(Default, Getter)]
    struct Foo {
        #[get(str, mut)]
        string_field: String,
    }

    let mut foo = Foo::default();

    foo.string_field_mut().push_str("foo");

    assert_eq!(foo.string_field(), "foo");
}

#[derive(Default, Getter)]
#[get(pub, copy)]
pub struct Unnamed(#[get(rename = "x")] usize, usize);

#[test]
fn test_unnamed_struct() {
    let unnamed = Unnamed(123, 456);

    assert_eq!(unnamed.x(), 123);
    assert_eq!(unnamed.arg1(), 456);
}
