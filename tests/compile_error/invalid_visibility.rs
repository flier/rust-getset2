use getset2::Getter;

#[derive(Getter)]
pub struct Struct {
    #[get("pub(other)")]
    field: usize,
}

fn main() {}
