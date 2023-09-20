use derive_more::{Deref, DerefMut};
use proc_macro_error::abort;
use syn::{parse_quote_spanned, spanned::Spanned, Block, Type};

use crate::{args, ty::TypeExt};

use super::{Context, Getter, MutGetter};

#[derive(Clone, Debug, Deref, DerefMut)]
pub struct SliceGetter(Getter);

impl SliceGetter {
    pub fn new(ctx: &Context) -> Self {
        let mut getter = Getter::new(ctx);

        getter.sig.output = {
            let inner_ty = ctx.slice_inner_ty();

            parse_quote_spanned! { ctx.field.ty.span() =>
                -> & [ #inner_ty ]
            }
        };
        getter.block = ctx.as_slice();

        Self(getter)
    }
}

#[derive(Clone, Debug, Deref, DerefMut)]
pub struct MutSliceGetter(MutGetter);

impl MutSliceGetter {
    pub fn new(ctx: &Context) -> Self {
        let mut getter = MutGetter::new(ctx);

        getter.sig.output = {
            let inner_ty = ctx.slice_inner_ty();

            parse_quote_spanned! { ctx.field.ty.span() =>
                -> &mut [ #inner_ty ]
            }
        };
        getter.block = ctx.as_mut_slice();

        Self(getter)
    }
}

impl Context<'_> {
    pub fn is_slice(&self) -> bool {
        if args::merge_bool(&self.field.args.slice, &self.struct_args.slice).unwrap_or_default() {
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

    pub fn is_mut_slice(&self) -> bool {
        if args::merge_bool(&self.field.args.mut_slice, &self.struct_args.mut_slice)
            .unwrap_or_default()
        {
            if self.field.ty.slice_inner_ty().is_some() {
                return true;
            }

            if self.field.args.mut_slice.is_some() {
                abort!(
                    self.field.ty.span(),
                    "#[get(mut_slice)] should be applied to a Vec<T> or an array [T; N] type"
                );
            }
        }

        false
    }

    pub fn as_slice(&self) -> Box<Block> {
        let field_name = self.field.name();

        if let Some(path) = self
            .field
            .args
            .slice
            .as_ref()
            .and_then(|arg| arg.args.as_ref())
        {
            parse_quote_spanned!(self.field.span() => {
                #path( self.#field_name )
            })
        } else {
            parse_quote_spanned!(self.field.span() => {
                self. #field_name .as_slice()
            })
        }
    }

    pub fn as_mut_slice(&self) -> Box<Block> {
        let field_name = self.field.name();

        if let Some(path) = self
            .field
            .args
            .mut_slice
            .as_ref()
            .and_then(|arg| arg.args.as_ref())
        {
            parse_quote_spanned!(self.field.span() => {
                #path( self.#field_name )
            })
        } else {
            parse_quote_spanned!(self.field.span() => {
                self. #field_name .as_slice()
            })
        }
    }

    pub fn slice_inner_ty(&self) -> Type {
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
