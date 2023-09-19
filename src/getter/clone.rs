use derive_more::{Deref, From};
use proc_macro2::TokenStream;
use quote::{quote_spanned, ToTokens, TokenStreamExt};
use syn::spanned::Spanned;

use crate::args;

use super::Getter;

#[derive(Clone, Debug, Deref, From)]
pub struct CloneGetter<'a>(&'a Getter<'a>);

impl<'a> ToTokens for CloneGetter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let vis = self.vis();
        let attrs = self.field.attrs;
        let constness = self.constness();
        let method_name = self.method_name();
        let ty = &self.field.ty;
        let field_name = self.field.name();

        tokens.append_all(quote_spanned! { self.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis #constness fn #method_name(&self) -> #ty {
                ::std::clone::Clone::clone(& self.#field_name)
            }
        })
    }
}

pub trait CloneableExt {
    fn is_cloneable(&self) -> bool;
}

impl CloneableExt for Getter<'_> {
    fn is_cloneable(&self) -> bool {
        args::merge(&self.field.args.clone, &self.struct_args.clone).unwrap_or_default()
    }
}
