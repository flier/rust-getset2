use getset2::{Getter, Setter};

#[derive(Getter, Setter)]
pub struct Struct {
    #[get(pub, copy)]
    #[set(pub, extend)]
    // #[set(extend)] is not allowed for a field that is not implements Extend trait.
    field: usize,
}

#[derive(Getter, Setter)]
#[get(pub, copy)]
#[set(pub, extend)] // #[set(extend)] can not be applied to the structure
pub struct Struct2 {
    field: usize,
}

fn main() {}
