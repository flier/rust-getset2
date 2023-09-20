use quote::format_ident;
use syn::{parse_quote_spanned, spanned::Spanned, ItemFn};

use crate::args;

use super::Context;

pub fn getter(ctx: &Context) -> ItemFn {
    let attrs = &ctx.field.attrs;
    let vis = ctx.vis();
    let constness = ctx.constness();
    let prefix = ctx.prefix().unwrap_or_default();
    let basename = ctx.field.basename().to_string();
    let suffix = ctx.suffix().unwrap_or_default();
    let method_name = format_ident!("{}{}{}", prefix, basename, suffix);
    let ty = &ctx.field.ty;
    let field_name = ctx.field.name();

    parse_quote_spanned! { ctx.field.span() =>
        #( #attrs )*
        #[inline(always)]
        #vis #constness fn #method_name( &self ) -> & #ty {
            & self. #field_name
        }
    }
}
pub fn mut_getter(ctx: &Context) -> ItemFn {
    let attrs = &ctx.field.attrs;
    let vis = ctx.vis();
    let prefix = ctx.prefix().unwrap_or_default();
    let basename = ctx.field.basename().to_string();
    let suffix = ctx.suffix().unwrap_or_default();
    let method_name = format_ident!("{}{}{}_mut", prefix, basename, suffix);
    let ty = ctx.field.ty.clone();
    let field_name = ctx.field.name();

    parse_quote_spanned! { ctx.field.span() =>
        #( #attrs )*
        #[inline(always)]
        #vis fn #method_name( &mut self ) -> &mut #ty {
            &mut self . #field_name
        }
    }
}

impl Context<'_> {
    pub fn is_mutable(&self) -> bool {
        args::merge_bool(&self.field.args.mutable, &self.struct_args.mutable).unwrap_or_default()
    }
}
