use derive_more::{Deref, DerefMut};
use proc_macro_error::abort;
use syn::{parse_quote_spanned, spanned::Spanned, Type};

use crate::args::AsBool;

use super::{Context, Getter, MutGetter};

#[derive(Clone, Debug, Deref, DerefMut)]
pub struct BorrowGetter(Getter);

impl BorrowGetter {
    pub fn new(ctx: &Context) -> Self {
        let mut getter = Getter::new(ctx);

        getter.sig.output = {
            let borrowed_ty = ctx.borrowed_ty();

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

        Self(getter)
    }
}

#[derive(Clone, Debug, Deref, DerefMut)]
pub struct BorrowMutGetter(MutGetter);

impl BorrowMutGetter {
    pub fn new(ctx: &Context) -> Self {
        let mut getter = MutGetter::new(ctx);

        getter.sig.output = {
            let borrowed_ty = ctx.borrowed_mut_ty();

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

        Self(getter)
    }
}

impl Context<'_> {
    pub fn is_borrow(&self) -> bool {
        self.field.args.borrow.bool()
    }

    pub fn is_borrow_mut(&self) -> bool {
        self.field.args.borrow_mut.bool()
    }

    pub fn borrowed_ty(&self) -> &Type {
        if let Some(ref arg) = self.field.args.borrow {
            &arg.args
        } else {
            abort!(
                self.field.span(),
                "#[get(borrow(..))] should have a Borrowed type"
            );
        }
    }

    pub fn borrowed_mut_ty(&self) -> &Type {
        if let Some(ref arg) = self.field.args.borrow_mut {
            &arg.args
        } else {
            abort!(
                self.field.span(),
                "#[get(borrow_mut(..))] should have a Borrowed type"
            );
        }
    }
}
