use derive_more::{Deref, From};
use syn::{parse_quote, parse_quote_spanned, spanned::Spanned, Block};

use crate::args;

use super::{Context, Getter};

#[derive(Clone, Debug, Deref, From)]
pub struct BytesGetter(Getter);

impl BytesGetter {
    pub fn new(ctx: &Context) -> Self {
        let mut getter = Getter::new(ctx);

        getter.sig.output = parse_quote! {
            -> &[u8]
        };
        getter.block = Box::new(ctx.as_bytes());

        Self(getter)
    }
}

impl Context<'_> {
    pub fn is_bytes(&self) -> bool {
        args::merge(&self.field.args.bytes, &self.struct_args.bytes).unwrap_or_default()
    }

    pub fn as_bytes(&self) -> Block {
        let field_name = self.field.name();

        if let Some(path) = self
            .field
            .args
            .bytes
            .as_ref()
            .and_then(|arg| arg.args.as_ref())
        {
            parse_quote_spanned!(self.field.span() => {
                #path( self.#field_name )
            })
        } else {
            parse_quote_spanned!(self.field.span() => {
                self.#field_name.as_bytes()
            })
        }
    }
}
