use getset2::Getter;

#[derive(Getter)]
pub struct Struct {
    #[get(bytes)]
    field: Foobar,
}

struct Foobar {}

fn main() {}
