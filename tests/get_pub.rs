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
    fn private() {
        let bar = Bar::default();

        assert_eq!(bar.private_field(), 0);
        assert_eq!(bar.pub_self_field(), 0);
    }

    #[test]
    fn get_pub() {
        let bar = Bar::default();

        assert_eq!(bar.pub_in_module_field(), 0);
    }
}

#[test]
fn public() {
    let foobar = foo::Bar::default();

    assert_eq!(foobar.public_field(), 0);
}

#[test]
fn get_pub() {
    let foobar = foo::Bar::default();

    assert_eq!(foobar.pub_field(), 0);
    assert_eq!(foobar.pub_crate_field(), 0);
    assert_eq!(foobar.pub_super_field(), 0);
}
