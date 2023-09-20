use derive_more::{Deref, DerefMut};
use proc_macro_error::abort;
use syn::{parse_quote, parse_quote_spanned, spanned::Spanned, Block};

use crate::{args, ty::TypeExt};

use super::{Context, Getter, MutGetter};

#[derive(Clone, Debug, Deref, DerefMut)]
pub struct StrGetter(Getter);

impl StrGetter {
    pub fn new(ctx: &Context) -> Self {
        let mut getter = Getter::new(ctx);

        getter.sig.output = parse_quote! { -> &str };
        getter.block = ctx.as_str();

        Self(getter)
    }
}

#[derive(Clone, Debug, Deref, DerefMut)]
pub struct MutStrGetter(MutGetter);

impl MutStrGetter {
    pub fn new(ctx: &Context) -> Self {
        let mut getter = MutGetter::new(ctx);

        getter.sig.output = parse_quote! { -> &mut str };
        getter.block = ctx.as_mut_str();

        Self(getter)
    }
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

    pub fn as_str(&self) -> Box<Block> {
        let field_name = self.field.name();

        if let Some(path) = self
            .field
            .args
            .str
            .as_ref()
            .and_then(|arg| arg.args.as_ref())
        {
            parse_quote_spanned! (self.field.span() => {
                #path (& self.#field_name)
            })
        } else {
            parse_quote_spanned! (self.field.span() => {
                ::std::string::String::as_str(& self.#field_name)
            })
        }
    }

    pub fn as_mut_str(&self) -> Box<Block> {
        let field_name = self.field.name();

        if let Some(path) = self
            .field
            .args
            .str
            .as_ref()
            .and_then(|arg| arg.args.as_ref())
        {
            parse_quote_spanned! (self.field.span() => {
                #path (& self.#field_name)
            })
        } else {
            parse_quote_spanned! (self.field.span() => {
                ::std::string::String::as_mut_str(&mut self.#field_name)
            })
        }
    }
}
