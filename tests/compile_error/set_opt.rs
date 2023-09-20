use getset2::Setter;

#[derive(Setter)]
pub struct Struct {
    #[set(opt)] // #[set(op)] is not allowed for a field that is not a Option<T>
    field: usize,
}

#[derive(Setter)]
#[set(opt)] // #[set(opt)] is ignored when it applied to the structure
pub struct Struct2 {
    field: usize,
}

fn main() {}
