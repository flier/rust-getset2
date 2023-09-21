use getset2::{Getter, Setter};

#[derive(Default, Getter, Setter)]
#[get(pub, const, copy)]
#[set(attrs("rustfmt", "clippy"))]
struct Foo {
    #[doc = "test"]
        #[rustfmt::skip]
        #[clippy::cyclomatic_complexity = "100"]
        bar: usize,
}

#[test]
fn set_attrs() {
    let mut foo = Foo::default();

    assert_eq!(foo.set_bar(123).bar(), 123);
}
