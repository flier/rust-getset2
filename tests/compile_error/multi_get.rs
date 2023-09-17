use getset2::Getter;

#[derive(Getter)]
#[get(copy)]
#[get(mut)]
pub struct Foo {
    /// multi #[get(..)] will be merged
    #[get("pub")]
    #[get(prefix = "with")]
    bar: usize,
}

fn main() {
    let mut foo = Foo { bar: 123 };

    assert_eq!(foo.with_bar(), 123);

    *foo.with_bar_mut() = 456;
}
