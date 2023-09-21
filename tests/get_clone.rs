use std::path::{Path, PathBuf};

use getset2::Getter;

#[derive(Getter)]
struct Foo<'a> {
    #[get(clone)]
    pub clone_field: PathBuf,

    #[get(clone)]
    pub clone_ref_field: &'a PathBuf,
}

#[test]
fn get_clone() {
    let p = PathBuf::from("/tmp");
    let foo = Foo {
        clone_field: PathBuf::from("/tmp"),
        clone_ref_field: &p,
    };

    assert_eq!(foo.clone_field(), PathBuf::from("/tmp"));
    assert_eq!(foo.clone_ref_field(), Path::new("/tmp"));
}
