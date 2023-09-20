use getset2::Setter;

#[derive(Setter)]
pub struct Struct {
    #[set(try_into)]
    field: f32,
}

fn main() {
    let mut s = Struct { field: 3.14 };

    s.set_field(123).unwrap();
}
