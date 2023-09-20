use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::abort;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Data, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed};

use crate::{args, field::Field};

use super::{Context, StructArgs};

pub fn expand(input: DeriveInput) -> TokenStream2 {
    let DeriveInput {
        attrs,
        ident,
        generics,
        data,
        ..
    } = input.clone();

    let (struct_args, struct_args_span, _) = args::extract::<StructArgs, _>(attrs, "set", None);

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

        let setters = fields.into_iter().enumerate().map(|(field_idx, field)| {
            Context::new(&struct_args, struct_args_span, Field::new(field, field_idx))
        });

        quote_spanned! { input.span() =>
            impl #impl_generics #ident #ty_generics #where_clause {
                #( #setters )*
            }
        }
    } else {
        abort!(input, "#[derive(Setter)] can only be applied to structure")
    }
}
