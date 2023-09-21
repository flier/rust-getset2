use syn::{parse_quote_spanned, ItemFn};

use crate::args;

use super::Context;

pub fn setter(ctx: &Context) -> ItemFn {
    let attrs = &ctx.field.attrs;
    let vis = ctx.vis();
    let method_name = ctx.method_name();
    let ty = ctx.field.ty.clone();
    let field_name = ctx.field.name();
    let arg_name = ctx.field.basename();

    parse_quote_spanned! { ctx.attr_span() =>
        #( #attrs )*
        #[inline(always)]
        #vis fn #method_name<ARG>(&mut self, #arg_name: ARG) -> &mut Self
        where
            ARG : ::std::convert::Into<#ty>
        {
            #field_name = ::std::convert::Into::into( #arg_name );
            self
        }
    }
}

impl Context<'_> {
    pub fn is_into(&self) -> bool {
        args::merge_bool(&self.field.args.into, &self.struct_args.into).unwrap_or_default()
    }
}
