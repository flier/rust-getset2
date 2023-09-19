use derive_more::{Deref, From};
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::{quote_spanned, ToTokens, TokenStreamExt};
use syn::{spanned::Spanned, Type};

use crate::{args::AsBool, ty::TypeExt};

use super::{Getter, MutGetter};

#[derive(Clone, Debug, Deref, From)]
pub struct OptionGetter<'a>(&'a Getter<'a>);

impl<'a> ToTokens for OptionGetter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let vis = self.vis();
        let attrs = self.field.attrs;
        let constness = self.constness();
        let method_name = self.method_name();
        let inner_ty = self.option_inner_ty();
        let field_name = self.field.name();

        tokens.append_all(quote_spanned! { self.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis #constness fn #method_name(&self) -> Option<& #inner_ty> {
                ::std::option::Option::as_ref(& self.#field_name)
            }
        })
    }
}

#[derive(Clone, Debug, Deref, From)]
pub struct MutOptionGetter<'a>(&'a MutGetter<'a>);

impl<'a> ToTokens for MutOptionGetter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let vis = self.vis();
        let attrs = self.field.attrs;
        let method_name = self.method_name();
        let inner_ty = self.option_inner_ty();
        let field_name = self.field.name();

        tokens.append_all(quote_spanned! { self.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis fn #method_name(&mut self) -> Option<&mut #inner_ty> {
                ::std::option::Option::as_mut(&mut self.#field_name)
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
            .field
            .args
            .opt
            .as_bool()
            .or(self.struct_args.opt.as_bool())
            .unwrap_or_default()
        {
            if self.field.ty.option_inner_ty().is_some() {
                return true;
            }

            if self.field.args.opt.as_bool().unwrap_or_default() {
                abort!(
                    self.field.ty.span(),
                    "#[get(opt)] should be applied to an Option type"
                );
            }
        }

        false
    }

    fn option_inner_ty(&self) -> Type {
        match self.field.ty.option_inner_ty() {
            Some(ty) => ty,
            None => {
                abort!(self.field.span(), "field should be an `Option` type");
            }
        }
    }
}
