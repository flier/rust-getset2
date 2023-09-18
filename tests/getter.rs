use std::{collections::HashMap, path::PathBuf};

mod foo {
    use std::{collections::HashMap, path::PathBuf};

    use getset2::Getter;

    #[derive(Default, Getter)]
    #[get(pub, prefix = "with", mut)]
    pub struct Foo {
        /// field will use the default visibility (pub), prefix (with) and mutable flags
        field: usize,

        /// field with prefix will override the default prefix
        #[get(prefix = "get", mut(false))]
        override_field: usize,
    }

    #[derive(Default, Getter)]
    pub struct Bar {
        private_field: usize,

        pub public_field: usize,

        #[get(pub)]
        pub_field: usize,

        #[get(pub(self), copy)]
        pub_self_field: usize,

        #[get(pub(crate), copy)]
        pub_crate_field: usize,

        #[get(pub(super), copy)]
        pub_super_field: usize,

        #[get(pub(in crate::foo))]
        pub_in_field: usize,

        #[get(mut)]
        mut_field: usize,

        #[get(prefix = "with")]
        pub prefix_field: usize,

        #[get(suffix = "num", copy)]
        pub suffix_field: usize,

        #[get(copy, mut)]
        pub copy_field: usize,

        #[get(clone, mut)]
        pub clone_field: PathBuf,

        #[get(opt, mut)]
        pub option_field: Option<HashMap<String, usize>>,

        #[get(slice, mut)]
        pub vec_field: Vec<u8>,

        #[get(slice, mut)]
        pub array_field: [u8; 4],
    }

    #[test]
    fn test_private_getter() {
        let bar = Bar::default();

        assert_eq!(bar.private_field(), &0);
        assert_eq!(bar.pub_self_field(), 0);
    }

    #[test]
    fn test_mut_getter() {
        let mut bar = Bar::default();

        *bar.mut_field_mut() = 1;
        assert_eq!(bar.mut_field(), &1);
    }

    #[test]
    fn test_pub_getter() {
        let bar = Bar::default();

        assert_eq!(bar.pub_in_field(), &0);
    }
}

#[test]
fn test_public_getter() {
    let foobar = foo::Bar::default();

    assert_eq!(foobar.public_field(), &0);
}

#[test]
fn test_pub_getter() {
    let foobar = foo::Bar::default();

    assert_eq!(foobar.pub_field(), &0);
    assert_eq!(foobar.pub_crate_field(), 0);
    assert_eq!(foobar.pub_super_field(), 0);
}

#[test]
fn test_mut_getter() {
    let mut foo = foo::Foo::default();

    *foo.with_field_mut() = 1;

    assert_eq!(foo.with_field(), &1);
}

#[test]
fn test_getter_with_prefix() {
    let foo = foo::Foo::default();

    assert_eq!(foo.with_field(), &0);
    assert_eq!(foo.get_override_field(), &0);

    let bar = foo::Bar::default();

    assert_eq!(bar.with_prefix_field(), &0);
}

#[test]
fn test_getter_with_suffix() {
    let bar = foo::Bar::default();

    assert_eq!(bar.suffix_field_num(), 0);
}

#[test]
fn test_copy_field() {
    let mut bar = foo::Bar::default();

    *bar.copy_field_mut() = 1;

    let p = bar.copy_field();

    *bar.copy_field_mut() = 2;

    assert_eq!(p, 1);
    assert_eq!(bar.copy_field(), 2);
}

#[test]
fn test_clone_field() {
    let mut bar = foo::Bar::default();

    bar.clone_field_mut().push("/tmp");

    let p = bar.clone_field();

    bar.clone_field_mut().push("clone");

    assert_eq!(p, PathBuf::from("/tmp"));
    assert_eq!(bar.clone_field(), PathBuf::from("/tmp/clone"));
}

#[test]
fn test_opt_field() {
    let mut bar = foo::Bar::default();

    let _ = bar.option_field.insert(HashMap::new());

    bar.option_field_mut()
        .unwrap()
        .insert("foo".to_owned(), 123);

    assert_eq!(bar.option_field().unwrap().get("foo"), Some(&123));
}

#[test]
fn test_slice_field() {
    let mut bar = foo::Bar::default();

    bar.vec_field = vec![0; 3];
    bar.vec_field_mut().copy_from_slice(&[1, 2, 3]);

    bar.array_field_mut()
        .copy_from_slice(0x12345678u32.to_le_bytes().as_slice());

    assert_eq!(bar.vec_field(), &[1, 2, 3]);
    assert_eq!(bar.array_field(), &[120, 86, 52, 18]);
}
