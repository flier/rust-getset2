use syn::{parse_quote_spanned, spanned::Spanned, ItemFn};

use crate::args;

use super::{copy, Context};

pub fn getter(ctx: &Context) -> ItemFn {
    let mut getter = copy::getter(ctx);

    getter.block = {
        let field_name = ctx.field.name();

        parse_quote_spanned!(ctx.field.span() => {
            ::std::clone::Clone::clone(& #field_name)
        })
    };

    getter
}

impl Context<'_> {
    pub fn is_cloneable(&self) -> bool {
        args::merge_bool(&self.field.args.clone, &self.struct_args.clone).unwrap_or_default()
    }
}
