use derive_more::{Deref, From};
use proc_macro2::TokenStream;
use quote::{quote_spanned, ToTokens, TokenStreamExt};
use syn::spanned::Spanned;

use super::{AsBool, Getter};

#[derive(Clone, Debug, Deref, From)]
pub struct BytesGetter<'a>(&'a Getter<'a>);

impl<'a> BytesGetter<'a> {
    fn as_bytes(&self) -> TokenStream {
        let field_name = self.field_name();

        if let Some(ref arg) = self.field_args.bytes {
            if let Some(ref path) = arg.args {
                return quote_spanned! { self.field.span() =>
                    #path(#field_name)
                };
            }
        }

        quote_spanned! { self.field.span() =>
            #field_name.as_bytes()
        }
    }
}

impl<'a> ToTokens for BytesGetter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let vis = self.vis();
        let attrs = self.field_attrs;
        let constness = self.constness();
        let method_name = self.method_name();
        let as_bytes = self.as_bytes();

        tokens.append_all(quote_spanned! { self.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis #constness fn #method_name(&self) -> &[u8] {
                #as_bytes
            }
        })
    }
}

pub trait BytesExt {
    fn is_bytes(&self) -> bool;
}

impl BytesExt for Getter<'_> {
    fn is_bytes(&self) -> bool {
        self.field_args
            .bytes
            .as_bool()
            .or(self.struct_args.bytes.as_bool())
            .unwrap_or_default()
    }
}
