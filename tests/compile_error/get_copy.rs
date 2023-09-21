use getset2::Getter;

#[derive(Getter)]
pub struct Struct {
    #[get(copy)] // #[get(copy)] is not allowed for a field that is not implements `Copy`
    field: Foobar,
}

#[derive(Getter)]
#[get(copy)] // #[get(copy)] is not allowed for a field that is not implements `Copy`
pub struct Struct2 {
    field: Foobar,
}

pub struct Foobar;

fn main() {}
