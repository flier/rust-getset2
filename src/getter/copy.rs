use derive_more::{Deref, From};
use proc_macro2::TokenStream;
use quote::{quote_spanned, ToTokens, TokenStreamExt};
use syn::spanned::Spanned;

use super::{AsBool, Getter};

#[derive(Clone, Debug, Deref, From)]
pub struct CopyGetter<'a>(&'a Getter<'a>);

impl<'a> ToTokens for CopyGetter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let vis = self.vis();
        let attrs = self.field_attrs;
        let constness = self.constness();
        let method_name = self.method_name();
        let ty = &self.field.ty;
        let field_name = self.field_name();

        tokens.append_all(quote_spanned! { self.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis #constness fn #method_name(&self) -> #ty {
                #field_name
            }
        })
    }
}

pub trait CopyableExt {
    fn is_copyable(&self) -> bool;
}

impl CopyableExt for Getter<'_> {
    fn is_copyable(&self) -> bool {
        self.field_args
            .copy
            .as_bool()
            .or(self.struct_args.copy.as_bool())
            .unwrap_or_default()
    }
}
