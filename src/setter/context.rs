use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Visibility;

use crate::{args, field::Field as BaseField};

use super::{Field, FieldArgs, IntoSetter, Setter, StructArgs};

#[derive(Clone, Debug)]
pub struct Context<'a> {
    pub struct_args: &'a StructArgs,
    pub field: Field,
}

impl<'a> Context<'a> {
    pub fn new(struct_args: &'a StructArgs, field: BaseField) -> Self {
        let (field_args, field_attrs): (FieldArgs, _) = args::extract(field.attrs.clone(), "set");

        Self {
            struct_args,
            field: Field::new(field, field_args, field_attrs),
        }
    }

    pub fn vis(&self) -> Visibility {
        args::vis(&self.field.args.vis, &self.struct_args.vis, &self.field.vis)
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

        if self.is_into() {
            IntoSetter::new(self).to_tokens(tokens)
        } else {
            Setter::new(self).to_tokens(tokens)
        }
    }
}
