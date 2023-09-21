use getset2::Getter;

#[derive(Default, Getter)]
struct Foo {
    /// `pub fn vec_field(&self) -> &[u8]`
    /// `pub fn vec_field_mut(&mut self) -> &mut [u8]`
    #[get(slice, mut_slice)]
    pub vec_field: Vec<u8>,

    /// `pub fn array_field(&self) -> &[u8]`
    /// `pub fn array_field_mut(&mut self) -> &mut [u8]`
    #[get(slice, mut_slice)]
    pub array_field: [u8; 4],
}

#[test]
fn get_slice() {
    let mut foo = Foo::default();

    foo.vec_field = vec![0; 3];
    foo.vec_field_mut().copy_from_slice(&[1, 2, 3]);

    foo.array_field_mut()
        .copy_from_slice(0x12345678u32.to_le_bytes().as_slice());

    assert_eq!(foo.vec_field(), &[1, 2, 3]);
    assert_eq!(foo.array_field(), &[120, 86, 52, 18]);
}
