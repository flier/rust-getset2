use derive_more::{Deref, From};
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::{quote_spanned, ToTokens, TokenStreamExt};
use syn::{spanned::Spanned, Type};

use crate::extract;

use super::{AsBool, Getter, MutGetter};

#[derive(Clone, Debug, Deref, From)]
pub struct OptionGetter<'a>(&'a Getter<'a>);

impl<'a> ToTokens for OptionGetter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let vis = self.vis();
        let attrs = self.field_attrs;
        let constness = self.constness();
        let method_name = self.method_name();
        let inner_ty = self.option_inner_ty();
        let field_name = self.field_name();

        tokens.append_all(quote_spanned! { self.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis #constness fn #method_name(&self) -> Option<& #inner_ty> {
                #field_name.as_ref()
            }
        })
    }
}

#[derive(Clone, Debug, Deref, From)]
pub struct MutOptionGetter<'a>(&'a MutGetter<'a>);

impl<'a> ToTokens for MutOptionGetter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let vis = self.vis();
        let attrs = self.field_attrs;
        let method_name = self.method_name();
        let inner_ty = self.option_inner_ty();
        let field_name = self.field_name();

        tokens.append_all(quote_spanned! { self.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis fn #method_name(&mut self) -> Option<&mut #inner_ty> {
                #field_name.as_mut()
            }
        })
    }
}

pub trait OptionExt {
    fn is_option(&self) -> bool;

    fn option_inner_ty(&self) -> Type;
}

impl OptionExt for Getter<'_> {
    fn is_option(&self) -> bool {
        if self
            .field_args
            .opt
            .as_bool()
            .or(self.struct_args.opt.as_bool())
            .unwrap_or_default()
        {
            if extract::option_inner_ty(&self.field.ty).is_some() {
                return true;
            }

            if self.field_args.opt.as_bool().unwrap_or_default() {
                abort!(
                    self.field.ty.span(),
                    "#[get(opt)] should be applied to an Option type"
                );
            }
        }

        false
    }

    fn option_inner_ty(&self) -> Type {
        match extract::option_inner_ty(&self.field.ty) {
            Some(ty) => ty,
            None => {
                abort!(self.field.span(), "field should be an `Option` type");
            }
        }
    }
}
