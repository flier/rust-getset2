use derive_more::{Deref, From};
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::{quote_spanned, ToTokens, TokenStreamExt};
use syn::spanned::Spanned;

use crate::ty::TypeExt;

use super::{AsBool, Getter, MutGetter};

#[derive(Clone, Debug, Deref, From)]
pub struct StrGetter<'a>(&'a Getter<'a>);

impl<'a> StrGetter<'a> {
    fn as_str(&self) -> TokenStream {
        let field_name = self.field_name();

        if let Some(ref arg) = self.field_args.str {
            if let Some(ref path) = arg.args {
                return quote_spanned! { self.field.span() =>
                    #path(#field_name)
                };
            }
        }

        quote_spanned! { self.field.span() =>
            #field_name.as_str()
        }
    }
}

impl<'a> ToTokens for StrGetter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let vis = self.vis();
        let attrs = self.field_attrs;
        let method_name = self.method_name();
        let as_str = self.as_str();

        tokens.append_all(quote_spanned! { self.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis fn #method_name(&self) -> &str {
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
        let attrs = self.field_attrs;
        let field_name = self.field_name();
        let method_name = self.method_name();

        tokens.append_all(quote_spanned! { self.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis fn #method_name(&mut self) -> &mut str {
                #field_name.as_mut_str()
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
            .field_args
            .str
            .as_bool()
            .or(self.struct_args.str.as_bool())
            .unwrap_or_default()
        {
            if self.field.ty.is_string() {
                return true;
            }

            if self.field_args.str.is_some() {
                abort!(
                    self.field.ty.span(),
                    "#[get(str)] should be applied to a String type"
                );
            }
        }

        false
    }
}
