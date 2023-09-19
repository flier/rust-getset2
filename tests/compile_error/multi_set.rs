use getset2::{Getter, Setter};

#[derive(Getter, Setter)]
#[get(copy)]
#[set(prefix = "change")]
#[set(pub)]
pub struct Foo {
    /// multi #[set(..)] will be merged
    #[get(pub(crate))]
    #[set(suffix = "to")]
    bar: usize,
}

fn main() {
    let mut foo = Foo { bar: 123 };

    assert_eq!(foo.change_bar_to(123).bar(), 123);
}
