use proc_macro_error::abort;
use syn::{parse_quote, parse_quote_spanned, spanned::Spanned, ItemFn};

use crate::{args, ty::TypeExt};

use super::{gen, Context};

pub fn getter(ctx: &Context) -> ItemFn {
    let mut getter = gen::getter(ctx);

    getter.sig.output = parse_quote! { -> &str };
    getter.block = {
        let field_name = ctx.field.name();

        if let Some(path) = ctx
            .field
            .args
            .str
            .as_ref()
            .and_then(|arg| arg.args.as_ref())
        {
            parse_quote_spanned! (ctx.field.span() => {
                #path (& self.#field_name)
            })
        } else {
            parse_quote_spanned! (ctx.field.span() => {
                ::std::string::String::as_str(& self.#field_name)
            })
        }
    };

    getter
}

pub fn mut_getter(ctx: &Context) -> ItemFn {
    let mut getter = gen::mut_getter(ctx);

    getter.sig.output = parse_quote! { -> &mut str };
    getter.block = {
        let field_name = ctx.field.name();

        if let Some(path) = ctx
            .field
            .args
            .str
            .as_ref()
            .and_then(|arg| arg.args.as_ref())
        {
            parse_quote_spanned! (ctx.field.span() => {
                #path (& self.#field_name)
            })
        } else {
            parse_quote_spanned! (ctx.field.span() => {
                ::std::string::String::as_mut_str(&mut self.#field_name)
            })
        }
    };

    getter
}

impl Context<'_> {
    pub fn is_str(&self) -> bool {
        if args::merge_bool(&self.field.args.str, &self.struct_args.str).unwrap_or_default() {
            if self.field.ty.is_string() {
                return true;
            }

            if self.field.args.str.is_some() {
                abort!(
                    self.field.ty.span(),
                    "#[get(str)] should be applied to a String type"
                );
            }
        }

        false
    }

    pub fn is_mut_str(&self) -> bool {
        if args::merge_bool(&self.field.args.mut_str, &self.struct_args.mut_str).unwrap_or_default()
        {
            if self.field.ty.is_string() {
                return true;
            }

            if self.field.args.mut_str.is_some() {
                abort!(
                    self.field.ty.span(),
                    "#[get(mut_str)] should be applied to a String type"
                );
            }
        }

        false
    }
}
