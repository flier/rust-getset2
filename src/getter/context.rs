use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Token, Visibility};

use crate::{args, field::Field as BaseField};

use super::{Field, FieldArgs, StructArgs};

#[derive(Clone, Debug)]
pub struct Context<'a> {
    pub struct_args: &'a StructArgs,
    pub field: Field,
}

impl<'a> Context<'a> {
    pub fn new(struct_args: &'a StructArgs, field: BaseField) -> Self {
        let (field_args, field_attrs): (FieldArgs, _) = args::extract(field.attrs.clone(), "get");

        Self {
            struct_args,
            field: Field::new(field, field_args, field_attrs),
        }
    }

    pub fn vis(&self) -> Visibility {
        args::vis(&self.field.args.vis, &self.struct_args.vis, &self.field.vis)
    }

    pub fn constness(&self) -> Option<Token![const]> {
        args::constness(&self.field.args.constness, &self.struct_args.constness)
    }

    pub fn prefix(&self) -> Option<String> {
        args::prefix(&self.field.args.prefix, &self.struct_args.prefix)
    }

    pub fn suffix(&self) -> Option<String> {
        args::suffix(&self.field.args.suffix, &self.struct_args.suffix)
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
