use derive_more::{Deref, DerefMut};
use syn::{parse_quote_spanned, spanned::Spanned};

use crate::args;

use super::{Context, Getter};

#[derive(Clone, Debug, Deref, DerefMut)]
pub struct CopyGetter(Getter);

impl CopyGetter {
    pub fn new(ctx: &Context) -> Self {
        let mut getter = Getter::new(ctx);

        getter.sig.output = {
            let ty = &ctx.field.ty;

            parse_quote_spanned! { ctx.field.ty.span() =>
                -> #ty
            }
        };

        getter.block = {
            let field_name = ctx.field.name();

            parse_quote_spanned! ( ctx.field.span() => {
                self.#field_name
            })
        };

        Self(getter)
    }
}

impl Context<'_> {
    pub fn is_copyable(&self) -> bool {
        args::merge(&self.field.args.copy, &self.struct_args.copy).unwrap_or_default()
    }
}
