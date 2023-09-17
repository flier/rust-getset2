use getset2::Getter;

#[derive(Getter)]
pub struct Struct {
    #[get(opt)]
    field: usize,
}

fn main() {}
