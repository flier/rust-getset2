Derive `Setter` to generate the trivial setters base on the fields in a structure.

- [Example](#example)
- [Helper attributes](#helper-attributes)
  - [Visibility](#visibility): `pub` attribute
  - [Naming](#naming): `prefix`, `suffix` and `rename` attributes
  - [Pass-through Attribute](#pass-through-attribute): `attr` attribute

# Example


# Helper attributes

| attribute | struct | field | description |
| --------- | ------ | ----- | ----------- |
| [attr(...)](#pass-through-attribute) | | ✔ | Set attributes on the setter |
| [attrs(...)](#pass-through-attribute) | ✔ | | Add attributes to passthrough allow list |
| [prefix = "..."](#naming) | ✔ | ✔ | Prepend a `prefix` to the setter name |
| [pub(...)](#visibility) | ✔ | ✔ | Change the visibility of setter |
| [rename = "...`](#naming) | | ✔ | Set the setter name |
| [suffix = "...`](#naming) | ✔ | ✔ | Append a `suffix` to the setter name |

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

## Pass-through Attribute

`#[derive(Setter)]` automatic copies doc comments and  well-known attributes `#[...]` from your fields to the according setter methods, if it is one of the following:

- `/// ...` or `#[doc = ...]` - Documentation comments
- `#[cfg(...)]` or `#[cfg_attr(...)]` - Conditional compilation
- `#[allow(...)]`, `#[deny(...)]`, `#[forbid(...)]` or `#[warn(...)]` — Alters the default lint level.
- `#[deprecated(...)]` — Generates deprecation notices.
- `#[must_use]` — Generates a lint for unused values.

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

The following code will be generated.

```rust,ignore
 impl Foo {
    ///test
    #[rustfmt::skip]
    #[clippy::cyclomatic_complexity = "100"]
    #[inline(always)]
    fn set_bar(&mut self, bar: usize) -> &mut Self {
        self.bar = bar;
        self
    }
}
```

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
