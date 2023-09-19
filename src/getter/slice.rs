use derive_more::{Deref, From};
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::{quote_spanned, ToTokens, TokenStreamExt};
use syn::{spanned::Spanned, Type};

use crate::{args, ty::TypeExt};

use super::{Getter, MutGetter};

#[derive(Clone, Debug, Deref, From)]
pub struct SliceGetter<'a>(&'a Getter<'a>);

impl<'a> ToTokens for SliceGetter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let vis = self.vis();
        let attrs = self.field.attrs;
        let constness = self.constness();
        let method_name = self.method_name();
        let inner_ty = self.slice_inner_ty();
        let as_slice = self.as_slice();

        tokens.append_all(quote_spanned! { self.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis #constness fn #method_name(&self) -> &[ #inner_ty ] {
                #as_slice
            }
        })
    }
}

#[derive(Clone, Debug, Deref, From)]
pub struct MutSliceGetter<'a>(&'a MutGetter<'a>);

impl<'a> ToTokens for MutSliceGetter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let vis = self.vis();
        let attrs = self.field.attrs;
        let method_name = self.method_name();
        let inner_ty = self.slice_inner_ty();
        let field_name = self.field.name();

        tokens.append_all(quote_spanned! { self.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis fn #method_name(&mut self) -> &mut[ #inner_ty ] {
                self.#field_name.as_mut_slice()
            }
        })
    }
}

pub trait SliceExt {
    fn is_slice(&self) -> bool;

    fn as_slice(&self) -> TokenStream;

    fn slice_inner_ty(&self) -> Type;
}

impl SliceExt for Getter<'_> {
    fn is_slice(&self) -> bool {
        if args::merge(&self.field.args.slice, &self.struct_args.slice).unwrap_or_default() {
            if self.field.ty.slice_inner_ty().is_some() {
                return true;
            }

            if self.field.args.slice.is_some() {
                abort!(
                    self.field.ty.span(),
                    "#[get(slice)] should be applied to a Vec<T> or an array [T; N] type"
                );
            }
        }

        false
    }

    fn as_slice(&self) -> TokenStream {
        let field_name = self.field.name();

        if let Some(ref arg) = self.field.args.slice {
            if let Some(ref path) = arg.args {
                return quote_spanned! { self.field.span() =>
                    #path( self.#field_name )
                };
            }
        }

        quote_spanned! { self.field.span() =>
            self.#field_name.as_slice()
        }
    }

    fn slice_inner_ty(&self) -> Type {
        match self.field.ty.slice_inner_ty() {
            Some(ty) => ty,
            None => {
                abort!(
                    self.field.span(),
                    "field should be `Vec<T>` or an array `[T; N]` type"
                );
            }
        }
    }
}
