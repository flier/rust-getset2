use syn::{parse_quote_spanned, spanned::Spanned, ItemFn};

use crate::args;

use super::{gen, Context};

pub fn getter(ctx: &Context) -> ItemFn {
    let mut getter = gen::getter(ctx);

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

    getter
}

impl Context<'_> {
    pub fn is_copyable(&self) -> bool {
        args::merge_bool(&self.field.args.copy, &self.struct_args.copy).unwrap_or_default()
    }
}
