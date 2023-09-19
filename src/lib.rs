#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, DeriveInput};

mod args;
mod getter;
mod setter;
mod ty;
mod vis;

#[doc = include_str!("../doc/getter.md")]
#[proc_macro_derive(Getter, attributes(get))]
#[proc_macro_error]
pub fn getter(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let expanded = self::getter::expand(input);

    expanded.into()
}

#[doc = include_str!("../doc/setter.md")]
#[proc_macro_derive(Setter, attributes(set))]
#[proc_macro_error]
pub fn setter(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let expanded = self::setter::expand(input);

    expanded.into()
}
