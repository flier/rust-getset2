use derive_more::{Constructor, Deref};
use proc_macro2::Span;
use syn::{parse_quote, Attribute, Expr, Ident};

use crate::{args, field::Field as BaseField, ty::TypeExt};

use super::FieldArgs;

#[derive(Clone, Debug, Constructor, Deref)]
pub struct Field {
    #[deref]
    pub field: BaseField,
    pub args: FieldArgs,
    pub args_span: Option<Span>,
    pub attrs: Vec<Attribute>,
}

impl Field {
    pub fn basename(&self) -> Ident {
        args::name(&self.args.rename, &self.field.ident, self.field.idx)
    }

    pub fn ref_name(&self) -> Expr {
        let ty = self.field.name();

        if self.field.ty.ref_elem_ty().is_none() {
            Expr::Reference(parse_quote! { & #ty })
        } else {
            Expr::Field(ty)
        }
    }

    pub fn ref_mut_name(&self) -> Expr {
        let ty = self.field.name();

        if self.field.ty.ref_elem_ty().is_none() {
            Expr::Reference(parse_quote! { &mut #ty })
        } else {
            Expr::Field(ty)
        }
    }
}
