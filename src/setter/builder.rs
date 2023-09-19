use derive_more::Constructor;
use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::{args, field::Field as BaseField};

use super::{Field, FieldArgs, Setter, StructArgs};

#[derive(Clone, Debug, Constructor)]
pub struct Builder<'a> {
    pub struct_args: &'a StructArgs,
    pub field: BaseField<'a>,
}

impl<'a> ToTokens for Builder<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let (field_args, field_attrs): (FieldArgs, _) = args::extract(&self.field.attrs, "set");

        let field = Field::new(&self.field, field_args, &field_attrs);

        if field.args.skip {
            return;
        }

        let setter = Setter::new(self, field);

        setter.to_tokens(tokens)
    }
}
