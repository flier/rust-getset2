use derive_more::{Deref, From};
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::{quote_spanned, ToTokens, TokenStreamExt};
use syn::{spanned::Spanned, Type};

use crate::extract;

use super::{AsBool, Getter, MutGetter};

#[derive(Clone, Debug, Deref, From)]
pub struct SliceGetter<'a>(&'a Getter<'a>);

impl<'a> ToTokens for SliceGetter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let vis = self.vis();
        let attrs = self.field_attrs;
        let inner_ty = self.slice_inner_ty();
        let method_name = self.method_name();
        let as_slice = self.as_slice();

        tokens.append_all(quote_spanned! { self.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis fn #method_name(&self) -> &[ #inner_ty ] {
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
        let attrs = self.field_attrs;
        let inner_ty = self.slice_inner_ty();
        let field_name = self.field_name();
        let method_name = self.method_name();

        tokens.append_all(quote_spanned! { self.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis fn #method_name(&mut self) -> &mut[ #inner_ty ] {
                #field_name.as_mut_slice()
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
        if self
            .field_args
            .slice
            .as_bool()
            .or(self.struct_args.slice.as_bool())
            .unwrap_or_default()
        {
            if extract::slice_inner_ty(&self.field.ty).is_some() {
                return true;
            }

            if self.field_args.slice.is_some() {
                abort!(
                    self.field.ty.span(),
                    "#[get(slice)] should be applied to a Vec<T> or an array [T; N] type"
                );
            }
        }

        false
    }

    fn as_slice(&self) -> TokenStream {
        let field_name = self.field_name();

        if let Some(ref arg) = self.field_args.slice {
            if let Some(ref path) = arg.args {
                return quote_spanned! { self.field.span() =>
                    #path(#field_name)
                };
            }
        }

        quote_spanned! { self.field.span() =>
            #field_name.as_slice()
        }
    }

    fn slice_inner_ty(&self) -> Type {
        match extract::slice_inner_ty(&self.field.ty) {
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
