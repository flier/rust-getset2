use getset2::Getter;

#[derive(Default, Getter)]
struct Foo {
    #[get(prefix = "with", copy, mut)]
    pub prefix_field: usize,
}

#[test]
fn get_prefix() {
    let mut foo = Foo::default();

    *foo.with_prefix_field_mut() = 1;

    assert_eq!(foo.with_prefix_field(), 1);
}
