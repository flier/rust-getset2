use getset2::Getter;

#[derive(Getter)]
pub struct Struct {
    #[get(clone)] // #[get(clone)] is not allowed for a field that is not implements `Clone`
    field: Foobar,
}

#[derive(Getter)]
#[get(clone)] // #[get(clone)] is not allowed for a field that is not implements `Clone`
pub struct Struct2 {
    field: Foobar,
}

pub struct Foobar;

fn main() {}
