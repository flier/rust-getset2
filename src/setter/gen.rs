use syn::{parse_quote_spanned, spanned::Spanned, ItemFn};

use super::Context;

pub fn setter(ctx: &Context) -> ItemFn {
    let attrs = &ctx.field.attrs;
    let vis = ctx.vis();
    let method_name = ctx.method_name();
    let ty = ctx.field.ty.clone();
    let field_name = ctx.field.name();
    let arg_name = ctx.field.basename();

    parse_quote_spanned! { ctx.field.span() =>
        #( #attrs )*
        #[inline(always)]
        #vis fn #method_name(&mut self, #arg_name: #ty) -> &mut Self {
            #field_name = #arg_name;
            self
        }
    }
}
