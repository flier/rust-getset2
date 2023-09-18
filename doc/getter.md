Derive `Getter` to generate the trivial getters base on the fields in a structure.

- [Example](#example)
- [Helper attributes](#helper-attributes)
  - [Visibility](#visibility): `pub` attribute
  - [Constness](#constness): `const` attribute
  - [Mutable](#mutable): `mut` attribute
  - [Naming](#naming): `prefix`, `suffix` and `rename` attributes
  - [Result Type](#result-type): `clone`, `copy` attributes
# Example


# Helper attributes

| attribute | struct | field | description |
| --------- | ------ | ----- | ----------- |
| `pub` | ✔ | ✔ | Change the visibility of getter |
| `const` | ✔ | ✔ | A `const` function is permitted to call from a const context |
| `mut` | ✔ | ✔ | Generating mutable getter |
| `prefix` | ✔ | ✔ | Prepend a `prefix` to the getter name |
| `suffix` | ✔ | ✔ | Append a `suffix` to the getter name |
| `rename` | | ✔ | Set the getter name |
| `clone` | ✔ | ✔ | Return a `cloned` value |
| `copy` | ✔ | ✔ | Return a `copied` value |

## Visibility

By default, all `getter` visibility is consistent with the `field`, but you can change that with the `pub` attribute.

| attribute | struct | field | description |
| --------- | ------ | ----- | ----------- |
| `pub` | ✔ | ✔ | public |
| `pub(self)` | ✔ | ✔ | private |
| `pub(crate)` | ✔ | ✔ | visible within the current crate. |
| `pub(super)` | ✔ | ✔ | visible to the parent module. |
| `pub(in <SimplePath>)` | ✔ | ✔ | visible to the given module. |

```rust
pub mod outer {
    pub mod inner {
        use getset2::Getter;

        #[derive(Default, Getter)]
        #[get(copy)]
        pub struct Bar {
            /// `fn private_field(&self) -> usize`
            private_field: usize,

            /// `pub fn public_field(&self) -> usize`
            pub public_field: usize,

            /// `pub fn pub_field(&self) -> usize`
            #[get(pub)]
            pub_field: usize,

            /// `pub(self) fn pub_self_field(&self) -> usize`
            #[get(pub(self))]
            pub_self_field: usize,

            /// `pub(crate) fn pub_crate_field(&self) -> usize`
            #[get(pub(crate))]
            pub_crate_field: usize,

            /// `pub(super) fn pub_super_field(&self) -> usize`
            #[get(pub(super))]
            pub_super_field: usize,

            /// `pub(in crate::foo) fn pub_in_module_field(&self) -> usize`
            #[get(pub(in crate::outer))]
            pub_in_module_field: usize,
        }

        pub fn get_private_field(bar: &Bar) -> usize {
            bar.private_field()
        }
    }

    pub fn get_pub_super_field(bar: &inner::Bar) -> usize {
        bar.pub_super_field()
    }

    pub fn get_pub_in_module_field(bar: &inner::Bar) -> usize {
        bar.pub_in_module_field()
    }
}

use self::outer::inner::Bar;

fn main() {
    let bar = Bar::default();

    assert_eq!(bar.public_field(), 0);
    assert_eq!(bar.pub_field(), 0);
    assert_eq!(bar.pub_crate_field(), 0);

    outer::inner::get_private_field(&bar);
    outer::get_pub_super_field(&bar);
    outer::get_pub_in_module_field(&bar);
}
```

## Mutable

Use the `mut` attribute can generate mutable getter base on the field.

```rust
use getset2::Getter;

#[derive(Default, Getter)]
#[get(copy)]
struct Foo {
    #[get(mut)]
    field: usize,
}

fn main() {
    let mut foo = Foo::default();

    *foo.field_mut() = 1;

    assert_eq!(foo.field(), 1);
}
```

The following code will be generated.

```rust,ignore
impl Foo {
    #[inline(always)]
    fn field(&self) -> usize {
        self.field
    }

    #[inline(always)]
    fn field_mut(&mut self) -> &mut usize {
        &mut self.field
    }
}
```

The `mut` version of the `getter` automatically takes the `_mut` suffix, and return a mutable reference.

## Constness

A `const fn` is a function that one is permitted to call from a const context.
Most trivial getters can be set to `const fn`, and the function is interpreted by the compiler at compile time.

```rust
use getset2::Getter;

#[derive(Default, Getter)]
#[get(pub, const, copy, mut)]
struct Foo {
    /// `pub const fn private_field(&self) -> usize`
    private_field: usize,

    /// `pub const fn copy_field(&self) -> usize`
    #[get(copy)]
    copy_field: usize,

    /// `clone` getter can't be constness after call `Clone::clone()`
    #[get(clone, const(false))]
    clone_field: usize,
}

fn main() {
    let mut foo = Foo::default();

    *foo.private_field_mut() = 123;
    assert_eq!(foo.private_field(), 123);

    *foo.copy_field_mut() = 456;
    assert_eq!(foo.copy_field(), 456);

    *foo.clone_field_mut() = 789;
    assert_eq!(foo.clone_field(), 789);
}
```

## Naming

By default, the getter will simply take the same name as the field, you can use `prefix`, `suffix` or `rename` attribute to customize it.

```rust
use getset2::Getter;

#[derive(Default, Getter)]
#[get(copy, mut)]
struct Foo {
    /// `fn with_prefix_field(&self) -> usize`
    #[get(prefix = "with")]
    pub prefix_field: usize,

    /// `fn suffix_field_num(&self) -> usize`
    #[get(suffix = "num")]
    pub suffix_field: usize,

    /// `fn x(&self) -> usize`
    #[get(rename = "x")]
    pub renamed_field: usize,
}

fn main() {
    let mut foo = Foo::default();

    *foo.with_prefix_field_mut() = 123;
    *foo.suffix_field_num_mut() = 456;
    *foo.x_mut() = 789;

    assert_eq!(foo.with_prefix_field(), 123);
    assert_eq!(foo.suffix_field_num(), 456);
    assert_eq!(foo.x(), 789);
}
```

## Result Type

By default, the getter will return a reference to the field, you can use `clone` or `copy` attribute to customize the result type.

```rust
use std::path::PathBuf;
use getset2::Getter;

#[derive(Default, Getter)]
#[get(pub, mut)]
struct Foo {
    /// `fn plain_field(&self) -> &usize`
    plain_field: usize,

    /// `fn clone_field(&self) -> PathBuf`
    #[get(clone)]
    clone_field: PathBuf,

    /// `fn copy_field(&self) -> usize`
    #[get(copy)]
    copy_field: usize,
}

fn main() {
    let mut foo = Foo::default();

    // the `mut` getter always return a mutable reference
    *foo.plain_field_mut() = 123;

    // by default, the result type is a reference of field type
    assert_eq!(foo.plain_field(), &123);

    // push a directory to the `clone_field`
    foo.clone_field_mut().push("/tmp");

    // `p` should be a cloned `PathBuf`
    let p = foo.clone_field();

    // push a filename to the `clone_field`
    foo.clone_field_mut().push("file");

    // the cloned `PathBuf` should not be impacted
    assert_eq!(p, PathBuf::from("/tmp"));
    // the `clone_field` has changed
    assert_eq!(foo.clone_field(), PathBuf::from("/tmp/file"));

    *foo.copy_field_mut() = 123;

    // the result type is `usize`, not `&usize`
    assert_eq!(foo.copy_field(), 123);
}
```