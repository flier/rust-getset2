use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, DeriveInput};

mod extract;
mod getter;
mod setter;
mod vis;

#[proc_macro_derive(Getter, attributes(get))]
#[proc_macro_error]
pub fn getter(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match self::getter::expand(input) {
        Ok(expaneded) => expaneded.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

#[proc_macro_derive(Setter, attributes(set))]
#[proc_macro_error]
pub fn setter(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let expanded = self::setter::expand(input);

    expanded.into()
}
