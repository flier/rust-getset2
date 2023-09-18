use getset2::Getter;

#[derive(Getter)]
pub struct Struct {
    #[get(slice)] // #[get(slice)] is not allowed for a field that is not a Vec<T> or array [T; N]
    field: usize,
}

#[derive(Getter)]
#[get(slice)] // #[get(slice)] is ignored when it applied to the structure
pub struct Struct2 {
    field: usize,
}

fn main() {}
