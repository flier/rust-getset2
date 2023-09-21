use std::collections::HashMap;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

use getset2::{Getter, Setter};

#[derive(Default, Getter, Setter)]
pub struct Foo<'a> {
    #[get(str)]
    #[set(extend)]
    string_field: String,

    #[get(slice)]
    #[set(extend)]
    vec_field: Vec<usize>,

    #[set(extend)]
    map_field: HashMap<usize, usize>,

    #[set(extend(&'a Path))]
    path_field: PathBuf,

    #[set(extend(P: AsRef<Path>))]
    p_field: PathBuf,

    #[get(skip)]
    #[set(skip)]
    phantom: PhantomData<&'a u8>,
}

#[test]
fn set_extend() {
    let mut foo = Foo::default();

    assert_eq!(
        foo.extend_string_field("foo".chars())
            .extend_string_field("bar".chars())
            .append_string_field('!')
            .string_field(),
        "foobar!"
    );

    assert_eq!(
        foo.extend_vec_field([1, 2, 3])
            .extend_vec_field([4, 5, 6])
            .append_vec_field(7)
            .vec_field(),
        [1, 2, 3, 4, 5, 6, 7]
    );

    assert_eq!(
        foo.extend_map_field(vec![(1, 2), (3, 4)])
            .append_map_field((5, 6))
            .map_field()
            .get(&5)
            .unwrap(),
        &6
    );

    assert_eq!(
        foo.extend_path_field([Path::new("/"), Path::new("foo")])
            .append_path_field(Path::new("bar"))
            .path_field(),
        Path::new("/foo/bar")
    );

    assert_eq!(
        foo.extend_p_field(["/", "foo"])
            .append_p_field("bar")
            .p_field(),
        Path::new("/foo/bar")
    )
}
