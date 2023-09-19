use derive_more::{Deref, From};
use proc_macro2::TokenStream;
use quote::{format_ident, quote_spanned, ToTokens, TokenStreamExt};
use syn::{spanned::Spanned, Ident};

use crate::args::AsBool;

use super::Getter;

#[derive(Clone, Debug, Deref, From)]
pub struct MutGetter<'a>(&'a Getter<'a>);

impl<'a> MutGetter<'a> {
    pub fn method_name(&self) -> Ident {
        let prefix = self.prefix().unwrap_or_default();
        let name = self.name();
        let suffix = self.suffix().unwrap_or_default();

        format_ident!("{}{}{}_mut", prefix, name.to_string(), suffix)
    }
}

impl<'a> ToTokens for MutGetter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let vis = self.vis();
        let attrs = self.field_attrs;
        let method_name = self.method_name();
        let ty = &self.field.ty;
        let field_name = self.field_name();

        tokens.append_all(quote_spanned! { self.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis fn #method_name(&mut self) -> &mut #ty {
                &mut self.#field_name
            }
        })
    }
}

pub trait MutableExt {
    fn as_mutable(&self) -> Option<MutGetter>;
}

impl MutableExt for Getter<'_> {
    fn as_mutable(&self) -> Option<MutGetter> {
        self.field_args
            .mutable
            .as_bool()
            .or(self.struct_args.mutable.as_bool())
            .and_then(|b| if b { Some(self.into()) } else { None })
    }
}
