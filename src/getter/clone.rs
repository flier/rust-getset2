use derive_more::{Deref, DerefMut};
use syn::{parse_quote_spanned, spanned::Spanned};

use crate::args;

use super::{Context, CopyGetter};

#[derive(Clone, Debug, Deref, DerefMut)]
pub struct CloneGetter(CopyGetter);

impl CloneGetter {
    pub fn new(ctx: &Context) -> Self {
        let mut getter = CopyGetter::new(ctx);

        getter.block = {
            let field_name = ctx.field.name();

            parse_quote_spanned!(ctx.field.span() => {
                ::std::clone::Clone::clone(& self.#field_name)
            })
        };

        Self(getter)
    }
}

impl Context<'_> {
    pub fn is_cloneable(&self) -> bool {
        args::merge(&self.field.args.clone, &self.struct_args.clone).unwrap_or_default()
    }
}
