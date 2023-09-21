use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::{format_ident, ToTokens};
use syn::{
    parse_quote, parse_quote_spanned, spanned::Spanned, GenericParam, Generics, ItemFn, Type,
    TypeParam,
};

use crate::ty::{self, TypeExt};

use super::Context;

pub fn setter(ctx: &Context) -> Setter {
    let attrs = &ctx.field.attrs;
    let vis = ctx.vis();
    let basename = ctx.field.basename().to_string();
    let extend_setter = format_ident!("extend_{}{}", &basename, ctx.suffix());
    let append_setter = format_ident!("append_{}{}", &basename, ctx.suffix());

    let (item_ty, extend_generic_param, append_generic): (
        _,
        Option<GenericParam>,
        Option<Generics>,
    ) = if let Some(extend) = ctx.field.args.extend() {
        use super::args::Extend::*;

        match extend {
            Type(ty) => (ty.clone(), None, None),
            Bound(param @ TypeParam { ident, .. }) => (
                parse_quote_spanned! { param.span() =>
                    #ident
                },
                Some(GenericParam::Type(param.clone())),
                Some(parse_quote_spanned! { param.span() =>
                    < #param >
                }),
            ),
        }
    } else {
        (ctx.extend_item_ty().clone(), None, None)
    };

    let extend_generic: Generics = {
        let params = Some(parse_quote_spanned! { ctx.field.ty.span() =>
            ITER: ::std::iter::IntoIterator<Item = #item_ty>
        })
        .into_iter()
        .chain(extend_generic_param);

        parse_quote_spanned! { ctx.field.ty.span() =>
            < #( #params ),* >
        }
    };

    let field_name = ctx.field.name();
    let arg_name = ctx.field.basename();

    Setter {
        extend: parse_quote_spanned! { ctx.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis fn #extend_setter #extend_generic (&mut self, #arg_name: ITER) -> &mut Self {
                #field_name.extend(#arg_name);
                self
            }
        },
        append: parse_quote_spanned! { ctx.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis fn #append_setter #append_generic (&mut self, #arg_name: #item_ty) -> &mut Self {
                #field_name.extend([ #arg_name ]);
                self
            }
        },
    }
}

pub struct Setter {
    extend: ItemFn,
    append: ItemFn,
}

impl ToTokens for Setter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.extend.to_tokens(tokens);
        self.append.to_tokens(tokens);
    }
}

const WELL_KNOWN_SEQ: &[&str] = &[
    "BinaryHeap",
    "BTreeSet",
    "HashSet",
    "LinkedList",
    "Vec",
    "VecDeque",
];

const WELL_KNOWN_MAP: &[&str] = &["HashMap", "BTreeMap"];

impl Context<'_> {
    pub fn is_extend(&self) -> bool {
        self.field.args.extend.is_some()
    }

    pub fn extend_item_ty(&self) -> Type {
        let ty = &self.field.ty;

        if ty.is_string() || self.field.ty.is_ref_string() {
            return parse_quote! { char };
        } else if let Some(args) = ty::generic_args_ty(ty, WELL_KNOWN_SEQ) {
            if let Some(ty) = args.into_iter().next() {
                return ty.clone();
            }
        } else if let Some(args) = ty::generic_args_ty(ty, WELL_KNOWN_MAP) {
            let mut iter = args.into_iter();

            if let Some((key_ty, value_ty)) = iter.next().zip(iter.next()) {
                return Type::Tuple(parse_quote! { (#key_ty, #value_ty) });
            }
        }

        abort!(
            self.field.ty.span(),
            "#[set(extend)] supports only some of the well-known types,
#[set(extend(Item))] should be used for a type which implements the `Extend<Item>` trait"
        )
    }
}
