use getset2::Getter;

#[derive(Getter)]
pub struct Struct {
    #[get(rename(?))]
    field: usize,
}

#[derive(Getter)]
pub struct Struct2 {
    #[get(rename("test"))]
    field: usize,
}

#[derive(Getter)]
pub struct Struct3 {
    #[get(rename)]
    field3: usize,
}

fn main() {}
