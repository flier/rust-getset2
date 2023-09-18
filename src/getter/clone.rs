use derive_more::{Deref, From};
use proc_macro2::TokenStream;
use quote::{quote_spanned, ToTokens, TokenStreamExt};
use syn::spanned::Spanned;

use super::{AsBool, Getter};

#[derive(Clone, Debug, Deref, From)]
pub struct CloneGetter<'a>(&'a Getter<'a>);

impl<'a> ToTokens for CloneGetter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let vis = self.vis();
        let attrs = self.field_attrs;
        let ty = &self.field.ty;
        let field_name = self.field_name();
        let method_name = self.method_name();

        tokens.append_all(quote_spanned! { self.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis fn #method_name(&self) -> #ty {
                #field_name.clone()
            }
        })
    }
}

pub trait CloneableExt {
    fn is_cloneable(&self) -> bool;
}

impl CloneableExt for Getter<'_> {
    fn is_cloneable(&self) -> bool {
        self.field_args
            .clone
            .as_bool()
            .or(self.struct_args.clone.as_bool())
            .unwrap_or_default()
    }
}
