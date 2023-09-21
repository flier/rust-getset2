use syn::{parse_quote, parse_quote_spanned, spanned::Spanned, ItemFn};

use crate::args;

use super::{gen, Context};

pub fn getter(ctx: &Context) -> ItemFn {
    let mut getter = gen::getter(ctx);

    getter.sig.output = parse_quote! {
        -> &[u8]
    };
    getter.block = Box::new({
        if let Some(path) = ctx.field.args.bytes_path() {
            let ref_field_name = ctx.field.ref_name();

            parse_quote_spanned!(ctx.field.span() => {
                #path( #ref_field_name )
            })
        } else {
            let field_name = ctx.field.name();

            parse_quote_spanned!(ctx.field.span() => {
                #field_name.as_bytes()
            })
        }
    });

    getter
}

impl Context<'_> {
    pub fn is_bytes(&self) -> bool {
        args::merge_bool(&self.field.args.bytes, &self.struct_args.bytes).unwrap_or_default()
    }
}
