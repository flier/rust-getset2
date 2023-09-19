use derive_more::{Constructor, Deref};
use proc_macro2::TokenStream;
use quote::{format_ident, quote_spanned, ToTokens, TokenStreamExt};
use syn::{spanned::Spanned, Ident, Visibility};

use crate::args;

use super::{Builder, Field};

#[derive(Clone, Debug, Constructor, Deref)]
pub struct Setter<'a> {
    #[deref]
    builder: &'a Builder<'a>,
    pub field: Field<'a>,
}

impl<'a> Setter<'a> {
    pub fn vis(&self) -> Visibility {
        args::vis(&self.field.args.vis, &self.struct_args.vis, &self.field.vis)
    }

    pub fn method_name(&self) -> Ident {
        let prefix = self.prefix().unwrap_or_else(|| "set_".to_string());
        let arg_name = self.field.arg_name();
        let suffix = self.suffix().unwrap_or_default();

        format_ident!("{}{}{}", prefix, arg_name.to_string(), suffix)
    }

    pub fn prefix(&self) -> Option<String> {
        args::prefix(&self.field.args.prefix, &self.struct_args.prefix)
    }

    pub fn suffix(&self) -> Option<String> {
        args::suffix(&self.field.args.suffix, &self.struct_args.suffix)
    }
}

impl<'a> ToTokens for Setter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let vis = self.vis();
        let attrs = self.field.attrs;
        let ty = &self.field.ty;
        let field_name = self.field.name();
        let method_name = self.method_name();
        let arg_name = self.field.arg_name();

        tokens.append_all(quote_spanned! { self.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis fn #method_name(&mut self, #arg_name: #ty) -> &mut Self {
                self.#field_name = #arg_name;
                self
            }
        })
    }
}
