use getset2::Getter;

#[derive(Getter)]
#[get(mut)]
struct Foo<'a> {
    ref_field: &'a usize,

    ref_mut_field: &'a mut usize,

    ref_ref_field: &'a &'a usize,
}

#[test]
fn get_ref() {
    let m = 0;
    let mut n = 0;
    let x = 123;
    let mut y = 456;
    let r = &m;

    let mut foo = Foo {
        ref_field: &m,
        ref_mut_field: &mut n,
        ref_ref_field: &r,
    };

    *foo.ref_field_mut() = &x;
    assert_eq!(foo.ref_field(), &&123);

    *foo.ref_mut_field_mut() = &mut y;
    assert_eq!(foo.ref_mut_field(), &&456);

    let r2 = &x;
    *foo.ref_ref_field_mut() = &r2;
    assert_eq!(foo.ref_ref_field(), &&&123);
}
