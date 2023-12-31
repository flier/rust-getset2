use merge::Merge;
use proc_macro2::Span;
use proc_macro_error::abort;
use quote::format_ident;
use structmeta::{Flag, NameArgs, NameValue};
use syn::{
    parse::Parse, parse_quote, spanned::Spanned, AttrStyle, Attribute, ExprPath, Ident, LitBool,
    LitStr, Token, Type, Visibility,
};

use crate::vis::{AsVisibility, Restricted};

pub trait AsBool {
    fn as_bool(&self) -> Option<bool>;

    fn bool(&self) -> bool {
        self.as_bool().unwrap_or_default()
    }
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

impl AsBool for Option<NameArgs<Option<ExprPath>>> {
    fn as_bool(&self) -> Option<bool> {
        self.as_ref().map(|_| true)
    }
}

impl AsBool for Option<NameArgs<Option<Type>>> {
    fn as_bool(&self) -> Option<bool> {
        self.as_ref().map(|_| true)
    }
}

impl AsBool for Option<NameArgs<Type>> {
    fn as_bool(&self) -> Option<bool> {
        self.as_ref().map(|_| true)
    }
}

pub fn merge_bool<L, R>(lhs: &L, rhs: &R) -> Option<bool>
where
    L: AsBool,
    R: AsBool,
{
    lhs.as_bool().or(rhs.as_bool())
}

pub fn merge_name_args<T: Clone>(
    lhs: &mut Option<NameArgs<Vec<T>>>,
    rhs: Option<NameArgs<Vec<T>>>,
) {
    if let Some(lhs) = lhs {
        if let Some(rhs) = rhs {
            lhs.args.extend(rhs.args);
        }
    } else {
        *lhs = rhs
    }
}

pub fn merge_flag(lhs: &mut Flag, rhs: Flag) {
    if rhs.span.is_some() {
        lhs.span = rhs.span
    }
}

const WELL_KNOWN_ATTRS: &[&str] = &[
    "allow",
    "cfg",
    "cfg_attr",
    "deny",
    "deprecated",
    "doc",
    "forbid",
    "must_use",
    "warn",
];

pub fn extract<T, I>(
    attrs: I,
    name: &str,
    allowed_attrs: Option<Vec<String>>,
) -> (T, Option<Span>, Vec<Attribute>)
where
    I: IntoIterator<Item = Attribute>,
    T: Default + Merge + Parse,
{
    let (args, attrs): (Vec<_>, Vec<_>) = attrs
        .into_iter()
        .partition(|attr| attr.path().is_ident(name));

    let (args, span) = parse_args(args, name);
    let attrs = extract_attrs(attrs, allowed_attrs);

    (args, span, attrs)
}

fn parse_args<T: Default + Merge + Parse>(args: Vec<Attribute>, name: &str) -> (T, Option<Span>) {
    args.into_iter()
        .map(|attr| match attr.parse_args::<T>() {
            Ok(args) => (args, attr.span()),
            Err(err) => {
                abort!(attr.span(), "invalid #[{}(..)] attribute, {}", name, err);
            }
        })
        .fold(
            (T::default(), None::<Span>),
            |(mut cur, span), (new, attr_span)| {
                cur.merge(new);
                (
                    cur,
                    if let Some(span) = span {
                        span.join(attr_span)
                    } else {
                        Some(attr_span)
                    },
                )
            },
        )
}

fn extract_attrs(attrs: Vec<Attribute>, allowed_attrs: Option<Vec<String>>) -> Vec<Attribute> {
    attrs
        .into_iter()
        .filter(|attr| {
            let ident = &attr.path().segments.first().unwrap().ident;

            attr.style == AttrStyle::Outer
                && (WELL_KNOWN_ATTRS.iter().any(|name| ident == name)
                    || allowed_attrs
                        .as_ref()
                        .map_or(false, |attrs| attrs.iter().any(|name| ident == name)))
        })
        .collect::<Vec<_>>()
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
    if merge_bool(field_constness, struct_constness).unwrap_or_default() {
        Some(parse_quote! { const })
    } else {
        None
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
