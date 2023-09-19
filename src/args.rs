use merge::Merge;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::{format_ident, quote};
use structmeta::{Flag, NameArgs, NameValue};
use syn::{
    parse::Parse, parse_quote, spanned::Spanned, AttrStyle, Attribute, Field, Ident, Index,
    LitBool, LitStr, Member, Path, Token, Type, Visibility,
};

use crate::vis::{AsVisibility, Restricted};

pub trait AsBool {
    fn as_bool(&self) -> Option<bool>;
}

impl AsBool for Flag {
    fn as_bool(&self) -> Option<bool> {
        self.span.map(|_| true)
    }
}

impl AsBool for Option<NameArgs<Option<LitBool>>> {
    fn as_bool(&self) -> Option<bool> {
        if let Some(v) = self {
            v.args.as_ref().map(|v| v.value).or(Some(true))
        } else {
            None
        }
    }
}

impl AsBool for Option<NameArgs<Option<Path>>> {
    fn as_bool(&self) -> Option<bool> {
        self.as_ref().map(|_| true)
    }
}

impl AsBool for Option<NameArgs<Type>> {
    fn as_bool(&self) -> Option<bool> {
        self.as_ref().map(|_| true)
    }
}

pub fn extract<'a, I, T>(attrs: I, name: &str) -> (T, Vec<&'a Attribute>)
where
    I: IntoIterator<Item = &'a Attribute>,
    T: Default + Parse + Merge,
{
    let (args, attrs): (Vec<_>, Vec<_>) = attrs
        .into_iter()
        .partition(|attr| attr.path().is_ident(name));

    let args = args
        .into_iter()
        .map(|attr| match attr.parse_args::<T>() {
            Ok(args) => args,
            Err(err) => {
                abort!(attr.span(), "invalid #[{}(..)] attribute, {}", name, err);
            }
        })
        .fold(T::default(), |mut cur, new| {
            cur.merge(new);
            cur
        });

    let attrs = attrs
        .into_iter()
        .filter(|attr| attr.style == AttrStyle::Outer && attr.path().is_ident("doc"))
        .collect::<Vec<_>>();

    (args, attrs)
}

pub fn vis(
    field_vis: &Option<NameArgs<Option<Restricted>>>,
    struct_vis: &Option<NameArgs<Option<Restricted>>>,
    default_vis: &Visibility,
) -> Visibility {
    field_vis
        .as_ref()
        .or(struct_vis.as_ref())
        .as_visibility()
        .unwrap_or_else(|| default_vis.clone())
}

pub fn constness(
    field_constness: &Option<NameArgs<Option<LitBool>>>,
    struct_constness: &Flag,
) -> Option<Token![const]> {
    if field_constness
        .as_bool()
        .or(struct_constness.as_bool())
        .unwrap_or_default()
    {
        Some(parse_quote! { const })
    } else {
        None
    }
}

pub fn field_name(field: &Field, idx: usize) -> TokenStream {
    match field.ident {
        Some(ref name) => quote! { #name },
        None => {
            let idx = Member::Unnamed(Index {
                index: idx as u32,
                span: field.span(),
            });

            quote! { #idx }
        }
    }
}

pub fn prefix(
    field_prefix: &Option<NameValue<LitStr>>,
    struct_prefix: &Option<NameValue<LitStr>>,
) -> Option<String> {
    field_prefix
        .as_ref()
        .or(struct_prefix.as_ref())
        .map(|s| format!("{}_", s.value.value()))
}

pub fn suffix(
    field_suffix: &Option<NameValue<LitStr>>,
    struct_suffix: &Option<NameValue<LitStr>>,
) -> Option<String> {
    field_suffix
        .as_ref()
        .or(struct_suffix.as_ref())
        .map(|s| format!("_{}", s.value.value()))
}

pub fn name(
    field_rename: &Option<NameArgs<Ident>>,
    field_ident: &Option<Ident>,
    field_idx: usize,
) -> Ident {
    field_rename
        .as_ref()
        .map(|arg| arg.args.clone())
        .unwrap_or_else(|| match field_ident {
            Some(ref name) => name.clone(),
            None => format_ident!("arg{}", field_idx),
        })
}
