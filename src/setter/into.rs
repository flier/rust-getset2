use derive_more::{Deref, DerefMut};
use quote::format_ident;
use syn::{parse_quote_spanned, spanned::Spanned, ItemFn};

use crate::args;

use super::Context;

#[derive(Clone, Debug, Deref, DerefMut)]
pub struct IntoSetter(ItemFn);

impl IntoSetter {
    pub fn new(ctx: &Context) -> Self {
        let attrs = &ctx.field.attrs;
        let vis = ctx.vis();
        let prefix = ctx.prefix().unwrap_or_else(|| "set_".to_string());
        let basename = ctx.field.basename().to_string();
        let suffix = ctx.suffix().unwrap_or_default();
        let method_name = format_ident!("{}{}{}", prefix, basename, suffix);
        let ty = ctx.field.ty.clone();
        let field_name = ctx.field.name();
        let arg_name = ctx.field.basename();

        Self(parse_quote_spanned! { ctx.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis fn #method_name<ARG: Into<#ty>>(&mut self, #arg_name: ARG) -> &mut Self {
                self.#field_name = ::std::convert::Into::into(#arg_name);
                self
            }
        })
    }
}

impl Context<'_> {
    pub fn is_into(&self) -> bool {
        args::merge_bool(&self.field.args.into, &self.struct_args.into).unwrap_or_default()
    }
}
