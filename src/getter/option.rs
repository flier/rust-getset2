use proc_macro_error::abort;
use syn::{parse_quote_spanned, spanned::Spanned, ItemFn, Type};

use crate::{
    args::{self, AsBool},
    ty::TypeExt,
};

use super::{gen, Context};

pub fn getter(ctx: &Context) -> ItemFn {
    let mut getter = gen::getter(ctx);

    getter.sig.output = {
        let inner_ty = ctx.option_inner_ty();

        parse_quote_spanned! { ctx.field.ty.span() =>
            -> Option<& #inner_ty>
        }
    };

    getter.block = {
        let field_name = ctx.field.name();

        parse_quote_spanned!( ctx.field.span() => {
            ::std::option::Option::as_ref(& #field_name)
        })
    };

    getter
}

pub fn mut_getter(ctx: &Context) -> ItemFn {
    let mut getter = gen::mut_getter(ctx);

    getter.sig.output = {
        let inner_ty = ctx.option_inner_ty();

        parse_quote_spanned! { ctx.field.ty.span() =>
            -> Option<&mut #inner_ty>
        }
    };

    getter.block = {
        let field_name = ctx.field.name();

        parse_quote_spanned!( ctx.field.span() => {
            ::std::option::Option::as_mut(&mut #field_name)
        })
    };

    getter
}

impl Context<'_> {
    pub fn is_option(&self) -> bool {
        if args::merge_bool(&self.field.args.opt, &self.struct_args.opt).unwrap_or_default() {
            if self.field.ty.option_inner_ty().is_some() {
                return true;
            }

            if self.field.args.opt.bool() {
                abort!(
                    self.field.ty.span(),
                    "#[get(opt)] should be applied to an `Option<T>` type"
                );
            }
        }

        false
    }

    pub fn option_inner_ty(&self) -> &Type {
        match self.field.ty.option_inner_ty() {
            Some(ty) => ty,
            None => {
                abort!(self.field.ty.span(), "field should be an `Option<T>` type");
            }
        }
    }
}
