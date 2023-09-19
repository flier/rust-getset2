use derive_more::{Constructor, Deref};
use proc_macro2::TokenStream;
use quote::{format_ident, quote_spanned, ToTokens, TokenStreamExt};
use syn::{spanned::Spanned, Attribute, Ident, Token, Visibility};

use crate::args;

use super::{FieldArgs, Getters};

#[derive(Clone, Debug, Constructor, Deref)]
pub struct Getter<'a> {
    #[deref]
    getters: &'a Getters<'a>,
    pub field_args: FieldArgs,
    pub field_attrs: &'a [&'a Attribute],
}

impl<'a> Getter<'a> {
    pub fn vis(&self) -> Visibility {
        args::vis(&self.field_args.vis, &self.struct_args.vis, &self.field.vis)
    }

    pub fn constness(&self) -> Option<Token![const]> {
        args::constness(&self.field_args.constness, &self.struct_args.constness)
    }

    pub fn method_name(&self) -> Ident {
        let prefix = self.prefix().unwrap_or_default();
        let name = self.name();
        let suffix = self.suffix().unwrap_or_default();

        format_ident!("{}{}{}", prefix, name.to_string(), suffix)
    }

    pub fn prefix(&self) -> Option<String> {
        args::prefix(&self.field_args.prefix, &self.struct_args.prefix)
    }

    pub fn suffix(&self) -> Option<String> {
        args::suffix(&self.field_args.suffix, &self.struct_args.suffix)
    }

    pub fn name(&self) -> Ident {
        args::name(&self.field_args.rename, &self.field.ident, self.field.idx)
    }
}

impl<'a> ToTokens for Getter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let vis = self.vis();
        let attrs = self.field_attrs;
        let ty = &self.field.ty;
        let field_name = self.field.name();
        let constness = self.constness();
        let method_name = self.method_name();

        tokens.append_all(quote_spanned! { self.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis #constness fn #method_name(&self) -> &#ty {
                & self.#field_name
            }
        })
    }
}
