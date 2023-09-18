use derive_more::Deref;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::{format_ident, quote_spanned, ToTokens, TokenStreamExt};
use syn::{
    parse_quote, parse_quote_spanned, spanned::Spanned, Attribute, Ident, Index, Member, Token,
    Visibility,
};

use super::{AsBool, FieldArgs, Getters};

#[derive(Clone, Debug, Deref)]
pub struct Getter<'a> {
    #[deref]
    getters: &'a Getters<'a>,
    pub field_args: FieldArgs,
    pub field_attrs: &'a [&'a Attribute],
}

impl<'a> Getter<'a> {
    pub fn new(
        getters: &'a Getters<'a>,
        field_args: FieldArgs,
        field_attrs: &'a [&'a Attribute],
    ) -> Self {
        Self {
            getters,
            field_args,
            field_attrs,
        }
    }

    pub fn vis(&self) -> Visibility {
        if let Some(arg) = self.field_args.public.as_ref() {
            if let Some(ref r) = arg.args {
                return r.clone().into();
            } else {
                return parse_quote_spanned! { self.field.span() => pub };
            }
        }

        if let Some(arg) = self.struct_args.public.as_ref() {
            if let Some(ref r) = arg.args {
                return r.clone().into();
            } else {
                return parse_quote_spanned! { self.field.span() => pub };
            }
        }

        return self.field.vis.clone();
    }

    pub fn constness(&self) -> Option<Token![const]> {
        if self
            .field_args
            .constness
            .as_bool()
            .or(self.struct_args.constness.as_bool())
            .unwrap_or_default()
        {
            Some(parse_quote! { const })
        } else {
            None
        }
    }

    pub fn field_name(&self) -> TokenStream {
        match self.field.ident {
            Some(ref name) => quote_spanned! { self.field.span() =>
                self.#name
            },
            None => {
                let idx = Member::Unnamed(Index {
                    index: self.field_idx as u32,
                    span: self.field.span(),
                });

                quote_spanned! { self.field.span() =>
                    self.#idx
                }
            }
        }
    }

    pub fn method_name(&self) -> Ident {
        let prefix = self.prefix().unwrap_or_default();
        let name = self.name();
        let suffix = self.suffix().unwrap_or_default();

        format_ident!("{}{}{}", prefix, name.to_string(), suffix)
    }

    pub fn prefix(&self) -> Option<String> {
        self.field_args
            .prefix
            .as_ref()
            .or(self.struct_args.prefix.as_ref())
            .map(|s| format!("{}_", s.value.value()))
    }

    pub fn suffix(&self) -> Option<String> {
        self.field_args
            .suffix
            .as_ref()
            .or(self.struct_args.suffix.as_ref())
            .map(|s| format!("_{}", s.value.value()))
    }

    pub fn name(&self) -> Ident {
        let rename = self.field_args.rename.as_ref().map(|s| match s.parse() {
            Ok(name) => name,
            Err(err) => {
                abort!(s.span(), "invalid field name to rename, {}", err);
            }
        });

        rename.unwrap_or_else(|| match self.field.ident {
            Some(ref name) => name.clone(),
            None => format_ident!("arg{}", self.field_idx),
        })
    }
}

impl<'a> ToTokens for Getter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let vis = self.vis();
        let attrs = self.field_attrs;
        let ty = &self.field.ty;
        let field_name = self.field_name();
        let constness = self.constness();
        let method_name = self.method_name();

        tokens.append_all(quote_spanned! { self.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis #constness fn #method_name(&self) -> &#ty {
                & #field_name
            }
        })
    }
}
