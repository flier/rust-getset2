Derive `Setter` to generate the trivial setters base on the fields in a structure.

- [Example](#example)
- [Helper attributes](#helper-attributes)
  - [Visibility](#visibility): `pub` attribute
  - [Naming](#naming): `prefix`, `suffix` and `rename` attributes
  - [Generic Setters](#generic-setters): `into` and `try_into` attribute
  - [Extend Collection](#extend-collection): `extend` attribute
  - [Pass-through Attribute](#pass-through-attribute): `attr` attribute
  - [Hidden Fields](#hidden-fields): `skip` attribute

# Example


# Helper attributes

| attribute | struct | field | description |
| --------- | ------ | ----- | ----------- |
| [attr(...)](#setattr) | | ✔ | Set attributes on the setter |
| [attrs(...)](#setattrs) | ✔ | | Add attributes to passthrough allow list |
| [into](#setinto) | ✔ | ✔ | Generating generic setter over the `Into` trait. |
| [prefix = "..."](#naming) | ✔ | ✔ | Prepend a `prefix` to the setter name |
| [pub(...)](#visibility) | ✔ | ✔ | Change the visibility of setter |
| [rename = "...`](#naming) | | ✔ | Set the setter name |
| [skip](#hidden-fields) | | ✔ | Skipping generate setter for the field |
| [suffix = "...`](#naming) | ✔ | ✔ | Append a `suffix` to the setter name |
| [try_into](#settry_into) | ✔ | ✔ | Generating generic setter over the

## Visibility

By default, all `setter` visibility is consistent with the `field`, but you can change that with the `pub` attribute.

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
        use getset2::{Getter, Setter};

        #[derive(Default, Getter, Setter)]
        #[get(copy)]
        pub struct Bar {
            /// `fn set_private_field(&mut self, private_field: usize) -> &mut Self`
            private_field: usize,

            /// `pub fn set_public_field(&mut self, public_field: usize) -> &mut Self`
            pub public_field: usize,

            /// `pub fn set_pub_field(&mut self, pub_field: usize) -> &mut Self`
            #[get(pub)]
            #[set(pub)]
            pub_field: usize,

            /// `pub(self) fn set_pub_self_field(&mut self, pub_self_field: usize) -> &mut Self`
            #[get(pub(self))]
            #[set(pub(self))]
            pub_self_field: usize,

            /// `pub(crate) fn set_pub_crate_field(&mut self, pub_crate_field: usize) -> &mut Self`
            #[get(pub(crate))]
            #[set(pub(crate))]
            pub_crate_field: usize,

            /// `pub(super) fn set_pub_super_field(&mut self, pub_super_field: usize) -> &mut Self`
            #[get(pub(super))]
            #[set(pub(super))]
            pub_super_field: usize,

            /// `pub(in crate::foo) fn set_pub_in_module_field(&mut self, pub_in_module_field: usize) -> &mut Self``
            #[get(pub(in crate::outer))]
            #[set(pub(in crate::outer))]
            pub_in_module_field: usize,
        }

        pub fn change_private_field(bar: &mut Bar, n: usize) -> usize {
            bar.set_private_field(n).private_field()
        }
    }

    pub fn change_pub_super_field(bar: &mut inner::Bar, n: usize) -> usize {
        bar.set_pub_super_field(n).pub_super_field()
    }

    pub fn change_pub_in_module_field(bar: &mut inner::Bar, n: usize) -> usize {
        bar.set_pub_in_module_field(n).pub_in_module_field()
    }
}

use self::outer::inner::Bar;

fn main() {
    let mut bar = Bar::default();

    assert_eq!(bar.set_public_field(123).public_field(), 123);
    assert_eq!(bar.set_pub_field(456).pub_field(), 456);
    assert_eq!(bar.set_pub_crate_field(789).pub_crate_field(), 789);

    assert_eq!(outer::inner::change_private_field(&mut bar, 123), 123);
    assert_eq!(outer::change_pub_super_field(&mut bar, 456), 456);
    assert_eq!(outer::change_pub_in_module_field(&mut bar, 789), 789);
}
```

## Naming

By default, the setter will simply take the same name as the field, you can use `prefix`, `suffix` or `rename` attribute to customize it.

```rust
use getset2::{Getter, Setter};

#[derive(Default, Getter, Setter)]
#[get(copy, mut)]
struct Foo {
    /// `fn with_prefix_field(&self, prefix_field: usize) -> &mut Self`
    #[set(prefix = "with")]
    pub prefix_field: usize,

    /// `fn set_suffix_field_num(&self, suffix_field: usize) -> &mut Self`
    #[set(suffix = "num")]
    pub suffix_field: usize,

    /// `fn set_x(&self, x: usize) -> &mut Self`
    #[get(rename(x))]
    #[set(rename(x))]
    pub renamed_field: usize,
}

fn main() {
    let mut foo = Foo::default();

    assert_eq!(foo.with_prefix_field(123).prefix_field(), 123);
    assert_eq!(foo.set_suffix_field_num(456).suffix_field(), 456);
    assert_eq!(foo.set_x(789).x(), 789);
}
```

## Generic Setters

### #[set(into)]

You can make each setter generic over the `Into` trait. It’s as simple as adding #[set(into)] to either a field or the whole structure.

```rust
use getset2::{Getter, Setter};

#[derive(Default, Getter, Setter)]
pub struct Foo {
    /// The generic getter will be generated
    ///
    /// ```
    /// fn set_into_field<ARG>(&mut self, into_field: ARG) -> &mut Self
    /// where
    ///     ARG: ::std::convert::Into<String>
    /// ```
    #[get(str)]
    #[set(into)]
    into_field: String,
}

let mut foo = Foo::default();

assert_eq!(foo.set_into_field("bar").into_field(), "bar");
```

### #[set(try_into)]

Alternatively, you can make each setter generic over the `TryInto` trait.

```rust
use getset2::{Getter, Setter};

#[derive(Default, Getter, Setter)]
pub struct Foo {
    /// The generic getter will be generated
    ///
    /// ```
    /// fn set_try_into_field<ARG>(
    ///     &mut self,
    ///     try_into_field: ARG,
    /// ) -> ::std::result::Result<&mut Self, <ARG as ::std::convert::TryInto<i32>>::Error>
    /// where
    ///     ARG: ::std::convert::TryInto<i32>,
    /// ```
    #[get(copy)]
    #[set(try_into)]
    try_into_field: i32,
}

fn main() {
    let mut foo = Foo::default();

    assert_eq!(foo.set_try_into_field(123).unwrap().try_into_field(), 123);
}
```

## Extend Collection

For collection types that implement the `Extend` trait, you can use `#[set(extend)]` directly to generate a setter that inserts values in bulk with `extend_` prefix, or add value one by one with `append_` prefix.

### #[set(extend)]

```rust
use std::collections::HashMap;

use getset2::{Getter, Setter};

#[derive(Default, Getter, Setter)]
pub struct Foo {
    #[get(str)]
    #[set(extend)]
    string_field: String,

    #[get(slice)]
    #[set(extend)]
    vec_field: Vec<usize>,

    #[set(extend)]
    map_field: HashMap<usize, usize>,
}

fn main() {
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

}
```

Value types are automatically recognized if the collection type is one of the following:
- BinaryHeap
- BTreeMap
- BTreeSet
- HashMap
- HashSet
- LinkedList
- String
- Vec
- VecDeque

Otherwise you need to explicitly specify in the `extend` attribute, like `#[set(extend(&'a Path))]`.

```rust
use std::path::{Path, PathBuf};

use getset2::{Getter, Setter};

#[derive(Default, Getter, Setter)]
pub struct Foo<'a> {
    #[set(extend(&'a Path))]
    path_field: PathBuf,

    #[get(skip)]
    #[set(skip)]
    phantom: std::marker::PhantomData<&'a u8>,
}

fn main() {
    let mut foo = Foo::default();

    assert_eq!(
        foo.extend_path_field([Path::new("/"), Path::new("foo")])
            .append_path_field(Path::new("bar"))
            .path_field(),
        Path::new("/foo/bar")
    );
}
```

The `extend` attribute also supports defining value type in a generic way, like `#[set(extend(P: AsRef<Path>))]`.

```rust
use std::path::{Path, PathBuf};

use getset2::{Getter, Setter};

#[derive(Default, Getter, Setter)]
pub struct Foo {
    #[set(extend(P: AsRef<Path>))]
    path_field: PathBuf,
}

fn main() {
    let mut foo = Foo::default();

    assert_eq!(
        foo.extend_path_field(["/", "foo"])
            .append_path_field("bar")
            .path_field(),
        Path::new("/foo/bar")
    );
}
```

## Pass-through Attribute

`#[derive(Setter)]` automatic copies doc comments and  well-known attributes `#[...]` from your fields to the according setter methods, if it is one of the following:

- `/// ...` or `#[doc = ...]` - Documentation comments
- `#[cfg(...)]` or `#[cfg_attr(...)]` - Conditional compilation
- `#[allow(...)]`, `#[deny(...)]`, `#[forbid(...)]` or `#[warn(...)]` — Alters the default lint level.
- `#[deprecated(...)]` — Generates deprecation notices.
- `#[must_use]` — Generates a lint for unused values.

### #[set(attr(...))]

In addition to that you can set attributes on setter using the `#[set(attr(...))]` attributes:

```rust
use getset2::{Getter, Setter};

#[derive(Default, Getter, Setter)]
#[get(pub, const, copy)]
struct Foo {
    /// some attribute
    #[set(attr(rustfmt::skip))]
    #[set(attr(clippy::cyclomatic_complexity = "100"))]
    bar: usize,
}
```

### #[set(attrs(...))]

You can also mark the name of the attribute to be passthrough directly on struct with `#[set(attrs(...))]` attribute.

```rust
use getset2::{Getter, Setter};

#[derive(Default, Getter, Setter)]
#[get(pub, const, copy)]
#[set(attrs("rustfmt", "clippy"))]
struct Foo {
    #[doc = "test"]
    #[rustfmt::skip]
    #[clippy::cyclomatic_complexity = "100"]
    bar: usize,
}
```

## Hidden Fields

### #[set(skip)]

You can hide fields by skipping their setters.

```rust
use getset2::{Getter, Setter};

#[derive(Getter, Setter)]
struct HiddenField {
    setter_present: u32,
    #[get(skip)]
    #[set(skip)]
    setter_skipped: u32,
}
```

The generation of skip getters and setters is set independently.
