use derive_more::{Constructor, Deref};
use syn::{Attribute, Ident};

use crate::{args, field::Field as BaseField};

use super::FieldArgs;

#[derive(Clone, Debug, Constructor, Deref)]
pub struct Field {
    #[deref]
    field: BaseField,
    pub args: FieldArgs,
    pub attrs: Vec<Attribute>,
}

impl Field {
    pub fn basename(&self) -> Ident {
        args::name(&self.args.rename, &self.field.ident, self.field.idx)
    }
}
