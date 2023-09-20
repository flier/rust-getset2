use getset2::{Getter, Setter};

#[derive(Getter, Setter)]
pub struct Struct {
    #[get(pub, copy)]
    #[set(pub, into)]
    field: f32,
}

fn main() {
    let mut s = Struct { field: 3.14 };

    assert_eq!(s.set_field(123i32).field(), 123.0);
}
