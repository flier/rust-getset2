#![allow(clippy::enum_variant_names)]

use structmeta::NameArgs;
use syn::{
    parse::{Parse, ParseStream},
    parse_quote, parse_quote_spanned,
    spanned::Spanned,
    Path, Token, Visibility,
};

#[derive(Clone, Debug)]
pub enum Restricted {
    PubSelf { self_token: Token![self] },
    PubSuper { super_token: Token![super] },
    PubCrate { crate_token: Token![crate] },
    PubInModule { in_token: Token![in], path: Path },
}

impl From<Restricted> for Visibility {
    fn from(restricted: Restricted) -> Self {
        match restricted {
            Restricted::PubSelf { self_token } => {
                parse_quote_spanned! { self_token.span() => pub(#self_token) }
            }
            Restricted::PubSuper { super_token } => {
                parse_quote_spanned! { super_token.span() => pub(#super_token) }
            }
            Restricted::PubCrate { crate_token } => {
                parse_quote_spanned! { crate_token.span() => pub(#crate_token) }
            }
            Restricted::PubInModule { in_token, path } => {
                parse_quote_spanned! { in_token.span() => pub(#in_token #path) }
            }
        }
    }
}

impl Parse for Restricted {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Token![self]) {
            Ok(Self::PubSelf {
                self_token: input.parse()?,
            })
        } else if lookahead.peek(Token![super]) {
            Ok(Self::PubSuper {
                super_token: input.parse()?,
            })
        } else if lookahead.peek(Token![crate]) {
            Ok(Self::PubCrate {
                crate_token: input.parse()?,
            })
        } else if lookahead.peek(Token![in]) {
            Ok(Self::PubInModule {
                in_token: input.parse()?,
                path: input.parse()?,
            })
        } else {
            Err(lookahead.error())
        }
    }
}

pub trait AsVisibility {
    fn as_visibility(&self) -> Option<Visibility>;
}

impl AsVisibility for Option<&NameArgs<Option<Restricted>>> {
    fn as_visibility(&self) -> Option<Visibility> {
        self.and_then(|arg| {
            arg.args
                .as_ref()
                .map(|r| r.clone().into())
                .or_else(|| Some(parse_quote! { pub }))
        })
    }
}
