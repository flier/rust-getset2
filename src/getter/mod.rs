#![allow(clippy::module_inception)]

mod args;
mod borrow;
mod builder;
mod bytes;
mod clone;
mod copy;
mod expand;
mod field;
mod getter;
mod mutable;
mod option;
mod slice;
mod str;

pub use self::args::{FieldArgs, StructArgs};
pub use self::borrow::{BorrowExt, BorrowGetter, BorrowMutGetter};
pub use self::builder::Builder;
pub use self::bytes::{BytesExt, BytesGetter};
pub use self::clone::{CloneGetter, CloneableExt};
pub use self::copy::{CopyGetter, CopyableExt};
pub use self::expand::expand;
pub use self::field::Field;
pub use self::getter::Getter;
pub use self::mutable::{MutGetter, MutableExt};
pub use self::option::{MutOptionGetter, OptionExt, OptionGetter};
pub use self::slice::{MutSliceGetter, SliceExt, SliceGetter};
pub use self::str::{MutStrGetter, StrExt, StrGetter};
