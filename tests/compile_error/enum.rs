use getset2::{Getter, Setter};

#[derive(Getter)]
pub enum Enum {
    Foo,
    Bar,
}

#[derive(Setter)]
pub enum Enum2 {
    Foo,
    Bar,
}

fn main() {}
