use getset2::{Getter, Setter};

#[derive(Default, Getter, Setter)]
#[get(pub, copy)]
pub struct Unnamed(
    #[get(rename(x))]
    #[set(rename(x))]
    usize,
    usize,
);

#[test]
fn unnamed_struct() {
    let mut unnamed = Unnamed::default();

    assert_eq!(unnamed.set_x(123).x(), 123);
    assert_eq!(unnamed.set_arg1(456).arg1(), 456);
}
