use derive_more::{Constructor, Deref};
use proc_macro2::TokenStream;
use quote::{format_ident, quote_spanned, ToTokens, TokenStreamExt};
use syn::{spanned::Spanned, Attribute, Ident, Visibility};

use crate::args;

use super::{FieldArgs, Setters};

#[derive(Clone, Debug, Constructor, Deref)]
pub struct Setter<'a> {
    #[deref]
    setters: &'a Setters<'a>,
    pub field_args: FieldArgs,
    pub field_attrs: &'a [&'a Attribute],
}

impl<'a> Setter<'a> {
    pub fn vis(&self) -> Visibility {
        args::vis(&self.field_args.vis, &self.struct_args.vis, &self.field.vis)
    }

    pub fn field_name(&self) -> TokenStream {
        args::field_name(self.field, self.field_idx)
    }

    pub fn method_name(&self) -> Ident {
        let prefix = self.prefix().unwrap_or_else(|| "set_".to_string());
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
        args::name(&self.field_args.rename, &self.field.ident, self.field_idx)
    }
}

impl<'a> ToTokens for Setter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let vis = self.vis();
        let attrs = self.field_attrs;
        let ty = &self.field.ty;
        let field_name = self.field_name();
        let method_name = self.method_name();
        let arg_name = self.name();

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
