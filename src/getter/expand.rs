use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Data, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed};

use crate::args;

use super::{Getters, StructArgs};

pub fn expand(input: DeriveInput) -> TokenStream {
    let (struct_args, _): (StructArgs, _) = args::extract(&input.attrs, "get");

    if let Data::Struct(DataStruct { fields, .. }) = &input.data {
        let name = &input.ident;
        let generics = &input.generics;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let fields = match fields {
            Fields::Named(FieldsNamed { named, .. }) => named,
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => unnamed,
            Fields::Unit => {
                return quote!();
            }
        };

        let getters = fields
            .into_iter()
            .enumerate()
            .map(|(field_idx, field)| Getters {
                struct_args: &struct_args,
                field,
                field_idx,
            });

        quote_spanned! { input.span() =>
            impl #impl_generics #name #ty_generics #where_clause {
                #( #getters )*
            }
        }
    } else {
        abort!(input, "#[derive(Getter)] can only be applied to structure")
    }
}
