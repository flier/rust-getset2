use proc_macro2::{Span, TokenStream};
use quote::{format_ident, ToTokens};
use syn::{parse_quote_spanned, spanned::Spanned, Ident, Visibility};

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
            args::extract::<FieldArgs, _>(field.attrs.clone(), "set", struct_args.allowed_attrs());

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

    #[allow(dead_code)]
    pub fn attr_span(&self) -> Span {
        self.field
            .args_span
            .or(self.struct_args_span)
            .unwrap_or(self.field.span())
    }

    pub fn vis(&self) -> Visibility {
        args::vis(&self.field.args.vis, &self.struct_args.vis, &self.field.vis)
    }
    pub fn method_name(&self) -> Ident {
        format_ident!(
            "{}{}{}",
            self.prefix(),
            self.field.basename(),
            self.suffix()
        )
    }

    pub fn prefix(&self) -> String {
        self.prefix_arg().unwrap_or_else(|| "set_".to_string())
    }

    pub fn with_prefix(&self, prefix: &str) -> String {
        self.prefix_arg().map_or_else(
            || format!("{}_", prefix),
            |arg| format!("{}_{}", arg, prefix),
        )
    }

    pub fn prefix_arg(&self) -> Option<String> {
        args::prefix(&self.field.args.prefix, &self.struct_args.prefix)
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

        if self.is_into() {
            super::into::setter(self).to_tokens(tokens)
        } else if self.is_try_into() {
            super::try_into::setter(self).to_tokens(tokens)
        } else if self.is_option() {
            super::option::setter(self).to_tokens(tokens)
        } else if self.is_extend() {
            super::extend::setter(self).to_tokens(tokens)
        } else {
            super::gen::setter(self).to_tokens(tokens)
        }
    }
}
