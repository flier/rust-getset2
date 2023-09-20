use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::{format_ident, quote_spanned, TokenStreamExt};
use syn::{parse_quote, spanned::Spanned, Type};

use crate::{
    args,
    ty::{self, TypeExt},
};

use super::Context;

pub fn setter(ctx: &Context, tokens: &mut TokenStream) {
    let attrs = &ctx.field.attrs;
    let vis = ctx.vis();
    let basename = ctx.field.basename().to_string();
    let extend_setter = format_ident!("extend_{}{}", &basename, ctx.suffix());
    let append_setter = format_ident!("append_{}{}", &basename, ctx.suffix());
    let item_ty = ctx.extend_item_ty();
    let field_name = ctx.field.name();
    let arg_name = ctx.field.basename();

    tokens.append_all(quote_spanned! { ctx.field.span() =>
        #( #attrs )*
        #[inline(always)]
        #vis fn #extend_setter<ITER: ::std::iter::IntoIterator<Item = #item_ty>>(&mut self, #arg_name: ITER) -> &mut Self {
            self.#field_name.extend(#arg_name);
            self
        }

        #( #attrs )*
        #[inline(always)]
        #vis fn #append_setter(&mut self, #arg_name: #item_ty) -> &mut Self {
            self.#field_name.extend([ #arg_name ]);
            self
        }
    })
}

impl Context<'_> {
    pub fn is_extend(&self) -> bool {
        args::merge_bool(&self.field.args.extend, &self.struct_args.extend).unwrap_or_default()
    }

    pub fn extend_item_ty(&self) -> Type {
        let ty = &self.field.ty;

        if let Some(ref ty) = self
            .field
            .args
            .extend
            .as_ref()
            .and_then(|arg| arg.args.clone())
        {
            return ty.clone();
        } else if ty.is_string() {
            return parse_quote! { char };
        } else if let Some(args) = ty::generic_args_ty(
            ty,
            [
                "BinaryHeap",
                "BTreeSet",
                "HashSet",
                "LinkedList",
                "Vec",
                "VecDeque",
            ],
        ) {
            if let Some(ty) = args.into_iter().next() {
                return ty.clone();
            }
        } else if let Some(args) = ty::generic_args_ty(ty, ["HashMap", "BTreeMap"]) {
            let mut iter = args.into_iter();

            if let Some((key_ty, value_ty)) = iter.next().zip(iter.next()) {
                return parse_quote! { (#key_ty, #value_ty) };
            }
        }

        abort!(ty.span(), "#[set(extend(..))] should have a Item type")
    }
}
