use getset2::Getter;

#[derive(Default, Getter)]
#[get(pub, const, mut)]
struct Foo {
    private_field: usize,

    #[get(copy)]
    copy_field: usize,

    #[get(clone, const(false))]
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
