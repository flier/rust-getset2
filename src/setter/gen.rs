use quote::format_ident;
use syn::{parse_quote_spanned, spanned::Spanned, ItemFn};

use super::Context;

pub fn setter(ctx: &Context) -> ItemFn {
    let attrs = &ctx.field.attrs;
    let vis = ctx.vis();
    let prefix = ctx.prefix().unwrap_or_else(|| "set_".to_string());
    let basename = ctx.field.basename().to_string();
    let suffix = ctx.suffix().unwrap_or_default();
    let method_name = format_ident!("{}{}{}", prefix, basename, suffix);
    let ty = ctx.field.ty.clone();
    let field_name = ctx.field.name();
    let arg_name = ctx.field.basename();

    parse_quote_spanned! { ctx.field.span() =>
        #( #attrs )*
        #[inline(always)]
        #vis fn #method_name(&mut self, #arg_name: #ty) -> &mut Self {
            self.#field_name = #arg_name;
            self
        }
    }
}
