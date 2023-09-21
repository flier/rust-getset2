use quote::quote;
use syn::{parse_quote, parse_quote_spanned, spanned::Spanned, ItemFn};

use crate::{args, ty::TypeExt};

use super::{gen, Context};

pub fn getter(ctx: &Context) -> ItemFn {
    let mut getter = gen::getter(ctx);

    getter.sig.output = parse_quote! {
        -> &[u8]
    };
    getter.block = Box::new({
        let field_name = ctx.field.name();

        if let Some(path) = ctx.field.args.bytes_path() {
            let ref_ = ctx.field.ty.ref_elem_ty().is_none().then(|| quote! { & });

            parse_quote_spanned!(ctx.field.span() => {
                #path(#ref_ self.#field_name )
            })
        } else {
            parse_quote_spanned!(ctx.field.span() => {
                self.#field_name.as_bytes()
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
