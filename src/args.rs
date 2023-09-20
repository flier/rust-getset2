use merge::Merge;
use proc_macro2::Span;
use proc_macro_error::abort;
use quote::format_ident;
use structmeta::{Flag, NameArgs, NameValue};
use syn::{
    parse::Parse, parse_quote, spanned::Spanned, AttrStyle, Attribute, Ident, LitBool, LitStr,
    Path, Token, Type, Visibility,
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

impl AsBool for Option<NameArgs<Option<Path>>> {
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
    T: Default + Parse + Merge,
{
    let (args, attrs): (Vec<_>, Vec<_>) = attrs
        .into_iter()
        .partition(|attr| attr.path().is_ident(name));

    let (args, span) = args
        .into_iter()
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
        );

    let all_allowed_attrs = allowed_attrs
        .unwrap_or_default()
        .iter()
        .cloned()
        .chain(WELL_KNOWN_ATTRS.iter().map(|&s| s.to_string()))
        .collect::<Vec<_>>();

    let attrs = attrs
        .into_iter()
        .filter(|attr| {
            attr.style == AttrStyle::Outer
                && all_allowed_attrs
                    .iter()
                    .any(|name| attr.path().segments.first().unwrap().ident == name)
        })
        .collect::<Vec<_>>();

    (args, span, attrs)
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
