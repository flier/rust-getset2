use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Field;

use crate::args;

use super::{FieldArgs, Setter, StructArgs};

#[derive(Clone, Debug)]
pub struct Setters<'a> {
    pub struct_args: &'a StructArgs,
    pub field: &'a Field,
    pub field_idx: usize,
}

impl<'a> ToTokens for Setters<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let (field_args, field_attrs): (FieldArgs, _) = args::extract(&self.field.attrs, "set");

        if field_args.skip {
            return;
        }

        let setter = Setter::new(self, field_args, field_attrs.as_slice());

        setter.to_tokens(tokens)
    }
}
