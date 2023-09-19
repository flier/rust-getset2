use derive_more::{Deref, DerefMut};
use quote::format_ident;
use syn::{parse_quote_spanned, spanned::Spanned, ItemFn};

use super::Context;

#[derive(Clone, Debug, Deref, DerefMut)]
pub struct Getter(ItemFn);

impl Getter {
    pub fn new(ctx: &Context) -> Self {
        let attrs = &ctx.field.attrs;
        let vis = ctx.vis();
        let constness = ctx.constness();
        let prefix = ctx.prefix().unwrap_or_default();
        let basename = ctx.field.basename().to_string();
        let suffix = ctx.suffix().unwrap_or_default();
        let method_name = format_ident!("{}{}{}", prefix, basename, suffix);
        let ty = &ctx.field.ty;
        let field_name = ctx.field.name();

        Self(parse_quote_spanned! { ctx.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis #constness fn #method_name( &self ) -> & #ty {
                & self. #field_name
            }
        })
    }
}
