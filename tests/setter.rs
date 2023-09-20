use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use getset2::{Getter, Setter};

mod foo {
    use getset2::{Getter, Setter};

    #[derive(Default, Getter, Setter)]
    #[get(copy)]
    pub struct Bar {
        private_field: usize,

        pub public_field: usize,

        #[get(pub)]
        #[set(pub)]
        pub_field: usize,

        #[get(pub(self))]
        #[set(pub(self))]
        pub_self_field: usize,

        #[get(pub(crate))]
        #[set(pub(crate))]
        pub_crate_field: usize,

        #[get(pub(super))]
        #[set(pub(super))]
        pub_super_field: usize,

        #[get(pub(in crate::foo))]
        #[set(pub(in crate::foo))]
        pub_in_module_field: usize,
    }

    #[test]
    fn test_private() {
        let mut bar = Bar::default();

        assert_eq!(bar.set_private_field(123).private_field(), 123);
        assert_eq!(bar.set_pub_self_field(456).pub_self_field(), 456);
    }

    #[test]
    fn test_set_pub() {
        let mut bar = Bar::default();

        assert_eq!(bar.set_pub_in_module_field(123).pub_in_module_field(), 123);
    }
}

#[test]
fn test_public() {
    let mut foobar = foo::Bar::default();

    assert_eq!(foobar.set_public_field(123).public_field(), 123);
}

#[test]
fn test_set_pub() {
    let mut foobar = foo::Bar::default();

    assert_eq!(foobar.set_pub_field(123).pub_field(), 123);
    assert_eq!(foobar.set_pub_crate_field(456).pub_crate_field(), 456);
    assert_eq!(foobar.set_pub_super_field(789).pub_super_field(), 789);
}

#[test]
fn test_unnamed_struct() {
    #[derive(Default, Getter, Setter)]
    #[get(pub, copy)]
    pub struct Unnamed(
        #[get(rename(x))]
        #[set(rename(x))]
        usize,
        usize,
    );

    let mut unnamed = Unnamed::default();

    assert_eq!(unnamed.set_x(123).x(), 123);
    assert_eq!(unnamed.set_arg1(456).arg1(), 456);
}

#[test]
fn test_set_into() {
    #[derive(Default, Getter, Setter)]
    pub struct Foo {
        #[get(str)]
        #[set(into)]
        into_field: String,
    }

    let mut foo = Foo::default();

    assert_eq!(foo.set_into_field("bar").into_field(), "bar");
}

#[test]
fn test_set_try_into() {
    #[derive(Default, Getter, Setter)]
    pub struct Foo {
        #[get(copy)]
        #[set(try_into)]
        try_into_field: i32,
    }

    let mut foo = Foo::default();

    assert_eq!(foo.set_try_into_field(123).unwrap().try_into_field(), 123);
}

#[test]
fn test_set_opt() {
    #[derive(Default, Getter, Setter)]
    pub struct Foo {
        #[set(opt)]
        option_field: Option<usize>,
    }

    let mut foo = Foo::default();

    assert_eq!(foo.set_option_field(123).option_field().unwrap(), 123);
}

#[test]
fn test_set_extend() {
    #[derive(Default, Getter, Setter)]
    pub struct Foo<'a> {
        #[get(str)]
        #[set(extend)]
        string_field: String,

        #[get(slice)]
        #[set(extend)]
        vec_field: Vec<usize>,

        #[set(extend)]
        map_field: HashMap<usize, usize>,

        #[set(extend(&'a Path))]
        path_field: PathBuf,

        #[set(extend(P: AsRef<Path>))]
        p_field: PathBuf,

        #[get(skip)]
        #[set(skip)]
        phantom: std::marker::PhantomData<&'a u8>,
    }

    let mut foo = Foo::default();

    assert_eq!(
        foo.extend_string_field("foo".chars())
            .extend_string_field("bar".chars())
            .append_string_field('!')
            .string_field(),
        "foobar!"
    );

    assert_eq!(
        foo.extend_vec_field([1, 2, 3])
            .extend_vec_field([4, 5, 6])
            .append_vec_field(7)
            .vec_field(),
        [1, 2, 3, 4, 5, 6, 7]
    );

    assert_eq!(
        foo.extend_map_field(vec![(1, 2), (3, 4)])
            .append_map_field((5, 6))
            .map_field()
            .get(&5)
            .unwrap(),
        &6
    );

    assert_eq!(
        foo.extend_path_field([Path::new("/"), Path::new("foo")])
            .append_path_field(Path::new("bar"))
            .path_field(),
        Path::new("/foo/bar")
    );

    assert_eq!(
        foo.extend_p_field(["/", "foo"])
            .append_p_field("bar")
            .p_field(),
        Path::new("/foo/bar")
    )
}
