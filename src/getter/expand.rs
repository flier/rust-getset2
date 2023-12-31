use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Data, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed};

use crate::{args, field::Field};

use super::{Context, StructArgs};

pub fn expand(input: DeriveInput) -> TokenStream {
    let DeriveInput {
        attrs,
        ident,
        generics,
        data,
        ..
    } = input.clone();

    let (struct_args, struct_args_span, _) = args::extract::<StructArgs, _>(attrs, "get", None);

    if let Data::Struct(DataStruct { fields, .. }) = data {
        let generics = generics;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let fields = match fields {
            Fields::Named(FieldsNamed { named, .. }) => named,
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => unnamed,
            Fields::Unit => {
                return quote!();
            }
        };

        let getters = fields.into_iter().enumerate().map(|(field_idx, field)| {
            Context::new(&struct_args, struct_args_span, Field::new(field, field_idx))
        });

        quote_spanned! { input.span() =>
            impl #impl_generics #ident #ty_generics #where_clause {
                #( #getters )*
            }
        }
    } else {
        abort!(input, "#[derive(Getter)] can only be applied to structure")
    }
}
