use getset2::Getter;

#[derive(Getter)]
pub struct Struct {
    #[get(slice)]
    field: usize,
}

fn main() {}
