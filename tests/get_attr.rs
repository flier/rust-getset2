use getset2::Getter;

#[derive(Default, Getter)]
#[get(pub, const, copy)]
struct Foo {
    #[doc = "test"]
    #[get(attr(rustfmt::skip))]
    #[get(attr(clippy::cyclomatic_complexity = "100"))]
    bar: usize,
}

#[test]
fn get_attr() {
    let foo = Foo { bar: 123 };

    assert_eq!(foo.bar(), 123);
}
