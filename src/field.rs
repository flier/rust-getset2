use derive_more::{Constructor, Deref};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, Index, Member};

#[derive(Clone, Debug, Constructor, Deref)]
pub struct Field {
    #[deref]
    pub field: syn::Field,
    pub idx: usize,
}

impl Field {
    pub fn name(&self) -> TokenStream {
        match self.field.ident {
            Some(ref name) => quote! { #name },
            None => {
                let idx = Member::Unnamed(Index {
                    index: self.idx as u32,
                    span: self.field.span(),
                });

                quote! { #idx }
            }
        }
    }
}
