use getset2::Getter;

#[derive(Default, Getter)]
#[get(pub, const, mut)]
struct Foo {
    #[get(clone)]
    clone_field: usize,
}

fn main() {}
