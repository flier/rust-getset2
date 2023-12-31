use std::collections::HashMap;

use getset2::Getter;

#[derive(Default, Getter)]
struct Foo {
    /// `pub fn option_field(&self) -> Option<&HashMap<String, usize>>`
    /// `pub fn option_field_mut(&mut self) -> Option<&mut HashMap<String, usize>>`
    #[get(opt, mut)]
    pub option_field: Option<HashMap<String, usize>>,
}

#[test]
fn get_opt() {
    let mut foo = Foo::default();

    let _ = foo.option_field.insert(HashMap::new());

    foo.option_field_mut()
        .unwrap()
        .insert("foo".to_owned(), 123);

    assert_eq!(foo.option_field().unwrap().get("foo"), Some(&123));
}
