use getset2::Getter;

#[derive(Getter)]
struct Foo<'a> {
    /// `fn copy_field(&self) -> usize`
    #[get(copy, mut)]
    copy_field: usize,

    /// `fn copy_ref_field(&self) -> usize`
    #[get(copy, mut)]
    copy_ref_field: &'a usize,
}

#[test]
fn get_copy() {
    let mut foo = Foo {
        copy_field: 0,
        copy_ref_field: &0,
    };

    *foo.copy_field_mut() = 1;
    assert_eq!(foo.copy_field(), 1);

    *foo.copy_ref_field_mut() = &2;
    assert_eq!(foo.copy_ref_field(), 2);
}
