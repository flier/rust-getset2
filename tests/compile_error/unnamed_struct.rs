use getset2::{Getter, Setter};

#[derive(Getter, Setter)]
#[get(copy)]
pub struct UnnamedStruct(u8);

fn main() {
    let mut u = UnnamedStruct(1);

    assert_eq!(u.set_arg0(2).arg0(), 2);
}
