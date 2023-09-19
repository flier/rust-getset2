use derive_more::{Deref, From};
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::{quote_spanned, ToTokens, TokenStreamExt};
use syn::{spanned::Spanned, Type};

use crate::args::AsBool;

use super::{Getter, MutGetter};

#[derive(Clone, Debug, Deref, From)]
pub struct BorrowGetter<'a>(&'a Getter<'a>);

impl<'a> ToTokens for BorrowGetter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let vis = self.vis();
        let attrs = self.field_attrs;
        let constness = self.constness();
        let method_name = self.method_name();
        let field_name = self.field_name();
        let borrowed_ty = self.borrowed_ty();

        tokens.append_all(quote_spanned! { self.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis #constness fn #method_name(&self) -> & #borrowed_ty {
                ::std::borrow::Borrow::borrow(& self.#field_name)
            }
        })
    }
}

#[derive(Clone, Debug, Deref, From)]
pub struct BorrowMutGetter<'a>(&'a MutGetter<'a>);

impl<'a> ToTokens for BorrowMutGetter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let vis = self.vis();
        let attrs = self.field_attrs;
        let method_name = self.method_name();
        let field_name = self.field_name();
        let borrowed_ty = self.borrowed_ty();

        tokens.append_all(quote_spanned! { self.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis fn #method_name(&mut self) -> &mut #borrowed_ty {
                ::std::borrow::BorrowMut::borrow_mut(&mut self.#field_name)
            }
        })
    }
}

pub trait BorrowExt {
    fn is_borrow(&self) -> bool;

    fn borrowed_ty(&self) -> &Type;
}

impl BorrowExt for Getter<'_> {
    fn is_borrow(&self) -> bool {
        self.field_args.borrow.as_bool().unwrap_or_default()
    }

    fn borrowed_ty(&self) -> &Type {
        if let Some(ref arg) = self.field_args.borrow {
            &arg.args
        } else {
            abort!(
                self.field.span(),
                "#[get(borrow(..))] should have a borrowed type"
            );
        }
    }
}
