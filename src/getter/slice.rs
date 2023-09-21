use proc_macro_error::abort;
use syn::{parse_quote_spanned, spanned::Spanned, ItemFn, Type};

use crate::{args, ty::TypeExt};

use super::{gen, Context};

pub fn getter(ctx: &Context) -> ItemFn {
    let mut getter = gen::getter(ctx);

    getter.sig.output = {
        let inner_ty = ctx.slice_inner_ty();

        parse_quote_spanned! { ctx.field.ty.span() =>
            -> & [ #inner_ty ]
        }
    };
    getter.block = {
        if let Some(path) = ctx.field.args.slice_path() {
            let ref_field_name = ctx.field.ref_name();

            parse_quote_spanned!(ctx.field.span() => {
                #path( #ref_field_name )
            })
        } else {
            let field_name = ctx.field.name();

            parse_quote_spanned!(ctx.field.span() => {
                #field_name .as_slice()
            })
        }
    };

    getter
}

pub fn mut_getter(ctx: &Context) -> ItemFn {
    let mut getter = gen::mut_getter(ctx);

    getter.sig.output = {
        let inner_ty = ctx.slice_inner_ty();

        parse_quote_spanned! { ctx.field.ty.span() =>
            -> &mut [ #inner_ty ]
        }
    };
    getter.block = {
        if let Some(path) = ctx.field.args.mut_slice_path() {
            let ref_mut_field_name = ctx.field.ref_mut_name();

            parse_quote_spanned!(ctx.field.span() => {
                #path( #ref_mut_field_name )
            })
        } else {
            let field_name = ctx.field.name();

            parse_quote_spanned!(ctx.field.span() => {
                #field_name .as_mut_slice()
            })
        }
    };

    getter
}

impl Context<'_> {
    pub fn is_slice(&self) -> bool {
        if args::merge_bool(&self.field.args.slice, &self.struct_args.slice).unwrap_or_default() {
            if self.field.ty.slice_inner_ty().is_some() || self.field.args.slice_path().is_some() {
                return true;
            }

            if self.field.args.slice.is_some() {
                abort!(
                    self.field.ty.span(),
                    "#[get(slice)] should be applied to a `Vec<T>` or an array `[T; N]` type"
                );
            }
        }

        false
    }

    pub fn is_mut_slice(&self) -> bool {
        if args::merge_bool(&self.field.args.mut_slice, &self.struct_args.mut_slice)
            .unwrap_or_default()
        {
            if self.field.ty.slice_inner_ty().is_some()
                || self.field.args.mut_slice_path().is_some()
            {
                return true;
            }

            if self.field.args.mut_slice.is_some() {
                abort!(
                    self.field.ty.span(),
                    "#[get(mut_slice)] should be applied to a `Vec<T>` or an array `[T; N]` type"
                );
            }
        }

        false
    }

    fn slice_inner_ty(&self) -> &Type {
        match self.field.ty.slice_inner_ty() {
            Some(ty) => ty,
            None => {
                abort!(
                    self.field.ty.span(),
                    "field should be `Vec<T>` or an array `[T; N]` type"
                );
            }
        }
    }
}
