use getset2::Getter;

#[derive(Default, Getter)]
#[get(pub, const, copy, attrs("rustfmt", "clippy"))]
struct Foo {
    #[doc = "test"]
        #[rustfmt::skip]
        #[clippy::cyclomatic_complexity = "100"]
        bar: usize,
}

#[test]
fn get_attrs() {
    let foo = Foo { bar: 123 };

    assert_eq!(foo.bar(), 123);
}
