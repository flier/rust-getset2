use getset2::Getter;

#[derive(Default, Getter)]
struct Foo {
    /// `fn field_mut(&mut self) -> &mut usize`
    #[get(mut, copy)]
    field: usize,
}

#[test]
fn get_mut() {
    let mut foo = Foo::default();

    *foo.field_mut() = 1;

    assert_eq!(foo.field(), 1);
}
