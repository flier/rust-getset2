use getset2::{Getter, Setter};

#[derive(Default, Getter, Setter)]
pub struct Foo {
    #[get(str)]
    #[set(into)]
    into_field: String,
}

#[test]
fn set_into() {
    let mut foo = Foo::default();

    assert_eq!(foo.set_into_field("bar").into_field(), "bar");
}
