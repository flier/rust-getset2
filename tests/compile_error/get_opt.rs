use getset2::Getter;

#[derive(Getter)]
pub struct Struct {
    #[get(opt)] // #[get(op)] is not allowed for a field that is not a Option<T>
    field: usize,
}

#[derive(Getter)]
#[get(opt)] // #[get(opt)] is ignored when it applied to the structure
pub struct Struct2 {
    field: usize,
}

fn main() {}
