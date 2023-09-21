use getset2::{Getter, Setter};

#[derive(Default, Getter, Setter)]
pub struct Foo {
    #[set(opt)]
    option_field: Option<usize>,
}

#[test]
fn set_opt() {
    let mut foo = Foo::default();

    assert_eq!(foo.set_option_field(123).option_field().unwrap(), 123);
}
