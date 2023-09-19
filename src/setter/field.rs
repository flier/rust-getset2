use derive_more::{Constructor, Deref};
use syn::{Attribute, Ident};

use crate::{args, field::Field as BaseField};

use super::FieldArgs;

#[derive(Clone, Debug, Constructor, Deref)]
pub struct Field<'a> {
    #[deref]
    field: &'a BaseField<'a>,
    pub args: FieldArgs,
    pub attrs: &'a [&'a Attribute],
}

impl<'a> Field<'a> {
    pub fn arg_name(&self) -> Ident {
        args::name(&self.args.rename, &self.field.ident, self.field.idx)
    }
}
