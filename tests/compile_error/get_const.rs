use getset2::Getter;

#[derive(Getter)]
pub struct Struct {
    #[get(const, clone)] // #[get(clone)] is not allowed to a `const` getter
    field: Foobar,
}

#[derive(Getter)]
#[get(const, clone)] // #[get(clone)] is not allowed to a `const` getter
pub struct Struct2 {
    field: Foobar,
}

#[derive(Clone)]
pub struct Foobar;

fn main() {}
