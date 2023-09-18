use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::DeriveInput;

pub fn expand(_input: DeriveInput) -> TokenStream2 {
    quote! {}
}
