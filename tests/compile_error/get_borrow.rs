use std::sync::Arc;

use getset2::Getter;

#[derive(Getter)]
pub struct Struct {
    #[get(borrow)] // #[get(borrow)] is not allowed for a field that is not a Borrow<T>
    field: usize,
}

#[derive(Getter)]
#[get(borrow)] // #[get(borrow)] is ignored when it applied to the structure
pub struct Struct1 {
    field: usize,
}

#[derive(Getter)]
pub struct Struct2 {
    #[get(borrow(usize))]
    field: usize,
}

#[derive(Getter)]
pub struct Struct3 {
    #[get(borrow(isize))] // #[get(borrow)] is not allowed for a field that is not a Borrow<T>
    field: Arc<usize>,
}

#[derive(Getter)]
pub struct Struct4 {
    #[get(borrow_mut(isize))]
    // #[get(borrow)] is not allowed for a field that is not a BorrowMut<T>
    field: Arc<usize>,
}

fn main() {}
