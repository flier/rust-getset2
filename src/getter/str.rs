use derive_more::{Deref, From};
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::{quote_spanned, ToTokens, TokenStreamExt};
use syn::{parse_quote, spanned::Spanned};

use crate::{args::AsBool, ty::TypeExt};

use super::{Getter, MutGetter};

#[derive(Clone, Debug, Deref, From)]
pub struct StrGetter<'a>(&'a Getter<'a>);

impl<'a> StrGetter<'a> {
    fn as_str(&self) -> TokenStream {
        let field_name = self.field.name();

        let path = self
            .field
            .args
            .str
            .clone()
            .and_then(|arg| arg.args)
            .unwrap_or_else(|| {
                parse_quote! {
                    ::std::string::String::as_str
                }
            });

        quote_spanned! { self.field.span() =>
            #path (& self.#field_name)
        }
    }
}

impl<'a> ToTokens for StrGetter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let vis = self.vis();
        let attrs = self.field.attrs;
        let constness = self.constness();
        let method_name = self.method_name();
        let as_str = self.as_str();

        tokens.append_all(quote_spanned! { self.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis #constness fn #method_name(&self) -> &str {
                #as_str
            }
        })
    }
}

#[derive(Clone, Debug, Deref, From)]
pub struct MutStrGetter<'a>(&'a MutGetter<'a>);

impl<'a> ToTokens for MutStrGetter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let vis = self.vis();
        let attrs = self.field.attrs;
        let method_name = self.method_name();
        let field_name = self.field.name();

        tokens.append_all(quote_spanned! { self.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis fn #method_name(&mut self) -> &mut str {
                ::std::string::String::as_mut_str(&mut self.#field_name)
            }
        })
    }
}

pub trait StrExt {
    fn is_str(&self) -> bool;
}

impl StrExt for Getter<'_> {
    fn is_str(&self) -> bool {
        if self
            .field
            .args
            .str
            .as_bool()
            .or(self.struct_args.str.as_bool())
            .unwrap_or_default()
        {
            if self.field.ty.is_string() {
                return true;
            }

            if self.field.args.str.is_some() {
                abort!(
                    self.field.ty.span(),
                    "#[get(str)] should be applied to a String type"
                );
            }
        }

        false
    }
}
