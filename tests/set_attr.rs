use getset2::{Getter, Setter};

#[derive(Default, Getter, Setter)]
#[get(pub, const, copy)]
struct Foo {
    #[doc = "test"]
    #[set(attr(rustfmt::skip))]
    #[set(attr(clippy::cyclomatic_complexity = "100"))]
    bar: usize,
}

#[test]
fn set_attr() {
    let mut foo = Foo::default();

    assert_eq!(foo.set_bar(123).bar(), 123);
}
