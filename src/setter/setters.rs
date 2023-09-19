use derive_more::Constructor;
use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::{args, field::Field};

use super::{FieldArgs, Setter, StructArgs};

#[derive(Clone, Debug, Constructor)]
pub struct Setters<'a> {
    pub struct_args: &'a StructArgs,
    pub field: Field<'a>,
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
