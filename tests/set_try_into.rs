use getset2::{Getter, Setter};

#[derive(Default, Getter, Setter)]
pub struct Foo {
    #[get(copy)]
    #[set(try_into)]
    try_into_field: i32,
}

#[test]
fn set_try_into() {
    let mut foo = Foo::default();

    assert_eq!(foo.set_try_into_field(123).unwrap().try_into_field(), 123);
}
