use derive_more::{Deref, DerefMut};
use proc_macro_error::abort;
use syn::{parse_quote_spanned, spanned::Spanned, Type};

use crate::{
    args::{self, AsBool},
    ty::TypeExt,
};

use super::{Context, Getter, MutGetter};

#[derive(Clone, Debug, Deref, DerefMut)]
pub struct OptionGetter(Getter);

impl OptionGetter {
    pub fn new(ctx: &Context) -> Self {
        let mut getter = Getter::new(ctx);

        getter.sig.output = {
            let inner_ty = ctx.option_inner_ty();

            parse_quote_spanned! { ctx.field.ty.span() =>
                -> Option<& #inner_ty>
            }
        };

        getter.block = {
            let field_name = ctx.field.name();

            parse_quote_spanned!( ctx.field.span() => {
                ::std::option::Option::as_ref(& self.#field_name)
            })
        };

        Self(getter)
    }
}

#[derive(Clone, Debug, Deref, DerefMut)]
pub struct MutOptionGetter(MutGetter);

impl MutOptionGetter {
    pub fn new(ctx: &Context) -> Self {
        let mut getter = MutGetter::new(ctx);

        getter.sig.output = {
            let inner_ty = ctx.option_inner_ty();

            parse_quote_spanned! { ctx.field.ty.span() =>
                -> Option<&mut #inner_ty>
            }
        };

        getter.block = {
            let field_name = ctx.field.name();

            parse_quote_spanned!( ctx.field.span() => {
                ::std::option::Option::as_mut(&mut self.#field_name)
            })
        };

        Self(getter)
    }
}

pub trait OptionExt {
    fn is_option(&self) -> bool;

    fn option_inner_ty(&self) -> Type;
}

impl OptionExt for Context<'_> {
    fn is_option(&self) -> bool {
        if args::merge_bool(&self.field.args.opt, &self.struct_args.opt).unwrap_or_default() {
            if self.field.ty.option_inner_ty().is_some() {
                return true;
            }

            if self.field.args.opt.bool() {
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
