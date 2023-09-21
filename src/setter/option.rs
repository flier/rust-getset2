use proc_macro_error::abort;
use quote::format_ident;
use syn::{parse_quote_spanned, spanned::Spanned, ItemFn, Type};

use crate::{
    args::{self, AsBool},
    ty::TypeExt,
};

use super::Context;

pub fn setter(ctx: &Context) -> ItemFn {
    let attrs = &ctx.field.attrs;
    let vis = ctx.vis();
    let basename = ctx.field.basename().to_string();
    let method_name = format_ident!("{}{}{}", ctx.prefix(), basename, ctx.suffix());
    let inner_ty = ctx.option_inner_ty();
    let field_name = ctx.field.name();
    let arg_name = ctx.field.basename();

    parse_quote_spanned! { ctx.field.span() =>
        #( #attrs )*
        #[inline(always)]
        #vis fn #method_name(&mut self, #arg_name: #inner_ty) -> &mut Self {
            #field_name = ::std::option::Option::Some( #arg_name );
            self
        }
    }
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
                    "#[set(opt)] should be applied to an `Option<T>` type"
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
