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
    fn test_private_setter() {
        let mut bar = Bar::default();

        assert_eq!(bar.set_private_field(123).private_field(), 123);
        assert_eq!(bar.set_pub_self_field(456).pub_self_field(), 456);
    }

    #[test]
    fn test_pub_setter() {
        let mut bar = Bar::default();

        assert_eq!(bar.set_pub_in_module_field(123).pub_in_module_field(), 123);
    }
}

#[test]
fn test_public_setter() {
    let mut foobar = foo::Bar::default();

    assert_eq!(foobar.set_public_field(123).public_field(), 123);
}

#[test]
fn test_pub_setter() {
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
fn test_into_setter() {
    #[derive(Default, Getter, Setter)]
    pub struct Foo {
        #[get(str)]
        #[set(into)]
        into_field: String,
    }

    let mut foo = Foo::default();

    assert_eq!(foo.set_into_field("bar").into_field(), "bar");
}
