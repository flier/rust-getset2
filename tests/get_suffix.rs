use getset2::Getter;

#[derive(Default, Getter)]
struct Foo {
    #[get(suffix = "num", copy, mut)]
    pub suffix_field: usize,
}

#[test]
fn get_suffix() {
    let mut foo = Foo::default();

    *foo.suffix_field_num_mut() = 1;

    assert_eq!(foo.suffix_field_num(), 1);
}
