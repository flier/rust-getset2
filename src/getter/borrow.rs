use proc_macro_error::abort;
use syn::{parse_quote_spanned, spanned::Spanned, ItemFn};

use crate::args::AsBool;

use super::{gen, Context};

pub fn getter(ctx: &Context) -> ItemFn {
    let mut getter = gen::getter(ctx);

    getter.sig.output = {
        let borrowed_ty = if let Some(ref arg) = ctx.field.args.borrow {
            &arg.args
        } else {
            abort!(
                ctx.attr_span(),
                "#[get(borrow(..))] should have a Borrowed type"
            );
        };

        parse_quote_spanned! { ctx.field.ty.span() =>
            -> & #borrowed_ty
        }
    };
    getter.block = {
        let field_name = ctx.field.name();

        parse_quote_spanned!(ctx.field.span() => {
            ::std::borrow::Borrow::borrow(& self.#field_name)
        })
    };

    getter
}

pub fn mut_getter(ctx: &Context) -> ItemFn {
    let mut getter = gen::mut_getter(ctx);

    getter.sig.output = {
        let borrowed_ty = if let Some(ref arg) = ctx.field.args.borrow_mut {
            &arg.args
        } else {
            abort!(
                ctx.attr_span(),
                "#[get(borrow_mut(..))] should have a Borrowed type"
            );
        };

        parse_quote_spanned! { ctx.field.ty.span() =>
            -> &mut #borrowed_ty
        }
    };
    getter.block = {
        let field_name = ctx.field.name();

        parse_quote_spanned!(ctx.field.span() => {
            ::std::borrow::BorrowMut::borrow_mut(&mut self.#field_name)
        })
    };

    getter
}

impl Context<'_> {
    pub fn is_borrow(&self) -> bool {
        self.field.args.borrow.bool()
    }

    pub fn is_borrow_mut(&self) -> bool {
        self.field.args.borrow_mut.bool()
    }
}
