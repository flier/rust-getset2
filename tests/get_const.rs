use getset2::Getter;

#[derive(Default, Getter)]
#[get(pub, const, mut)]
struct Foo {
    /// `pub const fn private_field(&self) -> &usize`
    private_field: usize,

    /// `pub const fn copy_field(&self) -> usize`
    #[get(copy)]
    copy_field: usize,

    /// `pub fn clone_field(&self) -> usize`
    #[get(clone, const(false))] // #[get(clone)] is not allowed to a `const` getter
    clone_field: usize,
}

#[test]
fn get_const() {
    let mut foo = Foo::default();

    *foo.private_field_mut() = 123;
    assert_eq!(foo.private_field(), &123);

    *foo.copy_field_mut() = 456;
    assert_eq!(foo.copy_field(), 456);

    *foo.clone_field_mut() = 789;
    assert_eq!(foo.clone_field(), 789);
}
