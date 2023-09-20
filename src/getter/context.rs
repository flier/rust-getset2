use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{parse_quote_spanned, spanned::Spanned, Token, Visibility};

use crate::{args, field::Field as BaseField};

use super::{Field, FieldArgs, StructArgs};

#[derive(Clone, Debug)]
pub struct Context<'a> {
    pub struct_args: &'a StructArgs,
    pub struct_args_span: Option<Span>,
    pub field: Field,
}

impl<'a> Context<'a> {
    pub fn new(
        struct_args: &'a StructArgs,
        struct_args_span: Option<Span>,
        field: BaseField,
    ) -> Self {
        let (field_args, field_args_span, mut field_attrs) =
            args::extract::<FieldArgs, _>(field.attrs.clone(), "get", struct_args.allowed_attrs());

        if let Some(meta) = field_args.attr.as_ref() {
            field_attrs.extend(meta.args.iter().map(|meta| {
                parse_quote_spanned! { meta.span() =>
                    #[ #meta ]
                }
            }));
        }

        Self {
            struct_args,
            struct_args_span,
            field: Field::new(field, field_args, field_args_span, field_attrs),
        }
    }

    pub fn attr_span(&self) -> Span {
        self.field
            .args_span
            .or(self.struct_args_span)
            .unwrap_or(self.field.span())
    }

    pub fn vis(&self) -> Visibility {
        args::vis(&self.field.args.vis, &self.struct_args.vis, &self.field.vis)
    }

    pub fn constness(&self) -> Option<Token![const]> {
        args::constness(&self.field.args.constness, &self.struct_args.constness)
    }

    pub fn prefix(&self) -> String {
        args::prefix(&self.field.args.prefix, &self.struct_args.prefix).unwrap_or_default()
    }

    pub fn suffix(&self) -> String {
        args::suffix(&self.field.args.suffix, &self.struct_args.suffix).unwrap_or_default()
    }
}

impl<'a> ToTokens for Context<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.field.args.skip {
            return;
        }

        if self.is_copyable() {
            super::copy::getter(self).to_tokens(tokens)
        } else if self.is_cloneable() {
            super::clone::getter(self).to_tokens(tokens)
        } else if self.is_option() {
            super::option::getter(self).to_tokens(tokens)
        } else if self.is_slice() {
            super::slice::getter(self).to_tokens(tokens)
        } else if self.is_str() {
            super::str::getter(self).to_tokens(tokens)
        } else if self.is_bytes() {
            super::bytes::getter(self).to_tokens(tokens)
        } else if self.is_borrow() {
            super::borrow::getter(self).to_tokens(tokens)
        } else {
            super::gen::getter(self).to_tokens(tokens)
        };

        if self.is_mutable() || self.is_mut_slice() || self.is_mut_str() || self.is_borrow_mut() {
            if self.is_option() {
                super::option::mut_getter(self).to_tokens(tokens)
            } else if self.is_mut_slice() {
                super::slice::mut_getter(self).to_tokens(tokens)
            } else if self.is_mut_str() {
                super::str::mut_getter(self).to_tokens(tokens)
            } else if self.is_borrow_mut() {
                super::borrow::mut_getter(self).to_tokens(tokens)
            } else {
                super::gen::mut_getter(self).to_tokens(tokens)
            }
        }
    }
}
