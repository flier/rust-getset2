use derive_more::Deref;
use merge::Merge;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use proc_macro_error::abort;
use quote::{format_ident, quote_spanned, ToTokens, TokenStreamExt};
use structmeta::{Flag, NameArgs, NameValue, StructMeta};
use syn::{
    spanned::Spanned, Attribute, Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed,
    LitBool, LitStr, Type, Visibility,
};

use crate::extract;

#[derive(Clone, Debug, Default, Merge, StructMeta)]
struct StructArgs {
    #[struct_meta(unnamed)]
    vis: Option<LitStr>,
    #[merge(strategy = merge_flag)]
    clone: Flag,
    #[merge(strategy = merge_flag)]
    copy: Flag,
    #[struct_meta(name = "mut")]
    #[merge(strategy = merge_flag)]
    mutable: Flag,
    #[struct_meta(name = "opt")]
    #[merge(strategy = merge_flag)]
    option: Flag,
    #[merge(strategy = merge_flag)]
    slice: Flag,
    prefix: Option<NameValue<Option<LitStr>>>,
    suffix: Option<NameValue<LitStr>>,
}

#[derive(Clone, Debug, Default, Merge, StructMeta)]
struct FieldArgs {
    #[struct_meta(unnamed)]
    vis: Option<LitStr>,
    #[merge(strategy = merge::bool::overwrite_false)]
    skip: bool,
    clone: Option<NameArgs<Option<LitBool>>>,
    copy: Option<NameArgs<Option<LitBool>>>,
    #[struct_meta(name = "mut")]
    mutable: Option<NameArgs<Option<LitBool>>>,
    #[struct_meta(name = "opt")]
    option: Option<NameArgs<Option<LitBool>>>,
    slice: Option<NameArgs<Option<LitStr>>>,
    rename: Option<LitStr>,
    prefix: Option<NameValue<Option<LitStr>>>,
    suffix: Option<NameValue<LitStr>>,
}

fn merge_flag(lhs: &mut Flag, rhs: Flag) {
    if rhs.span.is_some() {
        lhs.span = rhs.span
    }
}

pub fn expand(input: DeriveInput) -> syn::Result<TokenStream2> {
    let (struct_args, _): (StructArgs, _) = extract::args(&input.attrs, "get");

    if let Data::Struct(DataStruct {
        fields: Fields::Named(FieldsNamed { named, .. }),
        ..
    }) = &input.data
    {
        let name = &input.ident;
        let generics = &input.generics;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        let getters = named.into_iter().map(|field| {
            let gen = Getters {
                struct_args: &struct_args,
                field,
            };

            quote_spanned! { field.span() =>
                #gen
            }
        });

        Ok(quote_spanned! { input.span() =>
            impl #impl_generics #name #ty_generics #where_clause {
                #( #getters )*
            }
        })
    } else {
        abort!(
            input,
            "#[derive(Getter)] can only be applied to structure with named fields"
        )
    }
}

struct Getters<'a> {
    struct_args: &'a StructArgs,
    field: &'a Field,
}

impl<'a> ToTokens for Getters<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let (field_args, field_attrs): (FieldArgs, _) = extract::args(&self.field.attrs, "get");

        if field_args.skip {
            return;
        }

        let getter = Getter {
            struct_args: self.struct_args,
            field_args: &field_args,
            field_attrs: field_attrs.as_slice(),
            field: self.field,
        };

        if let Some(getter) = getter.as_copyable() {
            getter.to_tokens(tokens)
        } else if let Some(getter) = getter.as_cloneable() {
            getter.to_tokens(tokens)
        } else if let Some(getter) = getter.as_option() {
            getter.to_tokens(tokens)
        } else if let Some(getter) = getter.as_slice() {
            getter.to_tokens(tokens)
        } else {
            getter.to_tokens(tokens)
        };

        if let Some(getter) = getter.as_mutable() {
            if let Some(getter) = getter.as_mut_option() {
                getter.to_tokens(tokens)
            } else if let Some(getter) = getter.as_mut_slice() {
                getter.to_tokens(tokens)
            } else {
                getter.to_tokens(tokens)
            }
        }
    }
}

trait AsBool {
    fn as_bool(&self) -> bool;
}

impl AsBool for Flag {
    fn as_bool(&self) -> bool {
        self.span.is_some()
    }
}

impl AsBool for Option<NameArgs<Option<LitBool>>> {
    fn as_bool(&self) -> bool {
        self.as_ref()
            .map(|m| m.args.as_ref().map(|b| b.value).unwrap_or(true))
            .unwrap_or_default()
    }
}

#[derive(Clone, Debug)]
struct Getter<'a> {
    struct_args: &'a StructArgs,
    field_args: &'a FieldArgs,
    field_attrs: &'a [&'a Attribute],
    field: &'a Field,
}

impl<'a> Getter<'a> {
    const DEFAULT_FIELD_PREFIX: &'static str = "get_";

    fn as_copyable(&'a self) -> Option<CopyGetter<'a>> {
        if self.field_args.copy.as_bool() || self.struct_args.copy.as_bool() {
            Some(CopyGetter(self))
        } else {
            None
        }
    }

    fn as_cloneable(&'a self) -> Option<CloneGetter<'a>> {
        if self.field_args.clone.as_bool() || self.struct_args.clone.as_bool() {
            Some(CloneGetter(self))
        } else {
            None
        }
    }

    fn as_option(&'a self) -> Option<OptionGetter<'a>> {
        if self.is_option() {
            Some(OptionGetter(self))
        } else {
            None
        }
    }

    fn as_slice(&'a self) -> Option<SliceGetter<'a>> {
        if self.is_slice() {
            Some(SliceGetter(self))
        } else {
            None
        }
    }

    fn as_mutable(&'a self) -> Option<MutGetter<'a>> {
        if self.field_args.mutable.as_bool() || self.struct_args.mutable.as_bool() {
            Some(MutGetter(self))
        } else {
            None
        }
    }

    fn vis(&self) -> Visibility {
        self.field_args
            .vis
            .as_ref()
            .or(self.struct_args.vis.as_ref())
            .map(|s| match syn::parse_str(s.value().as_str()) {
                Ok(vis) => vis,
                Err(err) => {
                    abort!(s.span(), "invalid `visibility` for the field, {}", err);
                }
            })
            .unwrap_or_else(|| self.field.vis.clone())
    }

    fn field_name(&self) -> &Ident {
        match self.field.ident {
            Some(ref name) => name,
            None => abort!(self.field.span(), "field should have name"),
        }
    }

    fn method_name(&self) -> Ident {
        let prefix = self.prefix().unwrap_or_default();
        let name = self.name();
        let suffix = self.suffix().unwrap_or_default();

        format_ident!("{}{}{}", prefix, name.to_string(), suffix)
    }

    fn prefix(&self) -> Option<String> {
        self.field_args
            .prefix
            .as_ref()
            .or(self.struct_args.prefix.as_ref())
            .map(|s| {
                s.value
                    .as_ref()
                    .map_or(Self::DEFAULT_FIELD_PREFIX.to_string(), |s| {
                        format!("{}_", s.value())
                    })
            })
    }

    fn suffix(&self) -> Option<String> {
        self.field_args
            .suffix
            .as_ref()
            .or(self.struct_args.suffix.as_ref())
            .map(|s| format!("_{}", s.value.value()))
    }

    fn name(&self) -> Ident {
        let rename = self.field_args.rename.as_ref().map(|s| match s.parse() {
            Ok(name) => name,
            Err(err) => {
                abort!(s.span(), "invalid field name to rename, {}", err);
            }
        });

        match rename.or_else(|| self.field.ident.clone()) {
            Some(name) => name,
            None => abort!(self.field.span(), "field should have name"),
        }
    }

    fn is_option(&self) -> bool {
        if self.field_args.option.as_bool() || self.struct_args.option.as_bool() {
            if extract::option_inner_ty(&self.field.ty).is_some() {
                return true;
            }

            if self.field_args.option.as_bool() {
                abort!(
                    self.field.ty.span(),
                    "#[get(opt)] should be applied to an Option type"
                );
            }
        }

        false
    }

    fn option_inner_ty(&self) -> Type {
        match extract::option_inner_ty(&self.field.ty) {
            Some(ty) => ty,
            None => {
                abort!(self.field.span(), "field should be an `Option` type");
            }
        }
    }

    fn is_slice(&self) -> bool {
        if self.field_args.slice.is_some() || self.struct_args.slice.as_bool() {
            if extract::slice_inner_ty(&self.field.ty).is_some() {
                return true;
            }

            if self.field_args.slice.is_some() {
                abort!(
                    self.field.ty.span(),
                    "#[get(slice)] should be applied to a Vec<T> or an array [T; N] type"
                );
            }
        }

        false
    }

    fn slice_inner_ty(&self) -> Type {
        match extract::slice_inner_ty(&self.field.ty) {
            Some(ty) => ty,
            None => {
                abort!(
                    self.field.span(),
                    "field should be `Vec<T>` or an array `[T; N]` type"
                );
            }
        }
    }
}

impl<'a> ToTokens for Getter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let vis = self.vis();
        let attrs = self.field_attrs;
        let ty = &self.field.ty;
        let field_name = self.field_name();
        let method_name = self.method_name();

        tokens.append_all(quote_spanned! { self.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis fn #method_name(&self) -> &#ty {
                &self.#field_name
            }
        })
    }
}

#[derive(Clone, Debug, Deref)]
struct CopyGetter<'a>(&'a Getter<'a>);

impl<'a> ToTokens for CopyGetter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let vis = self.vis();
        let attrs = self.field_attrs;
        let ty = &self.field.ty;
        let field_name = self.field_name();
        let method_name = self.method_name();

        tokens.append_all(quote_spanned! { self.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis fn #method_name(&self) -> #ty {
                self.#field_name
            }
        })
    }
}

#[derive(Clone, Debug, Deref)]
struct CloneGetter<'a>(&'a Getter<'a>);

impl<'a> ToTokens for CloneGetter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let vis = self.vis();
        let attrs = self.field_attrs;
        let ty = &self.field.ty;
        let field_name = self.field_name();
        let method_name = self.method_name();

        tokens.append_all(quote_spanned! { self.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis fn #method_name(&self) -> #ty {
                self.#field_name.clone()
            }
        })
    }
}

#[derive(Clone, Debug, Deref)]
struct OptionGetter<'a>(&'a Getter<'a>);

impl<'a> ToTokens for OptionGetter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let vis = self.vis();
        let attrs = self.field_attrs;
        let inner_ty = self.option_inner_ty();
        let field_name = self.field_name();
        let method_name = self.method_name();

        tokens.append_all(quote_spanned! { self.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis fn #method_name(&self) -> Option<& #inner_ty> {
                self.#field_name.as_ref()
            }
        })
    }
}

#[derive(Clone, Debug, Deref)]
struct SliceGetter<'a>(&'a Getter<'a>);

impl<'a> ToTokens for SliceGetter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let vis = self.vis();
        let attrs = self.field_attrs;
        let inner_ty = self.slice_inner_ty();
        let field_name = self.field_name();
        let method_name = self.method_name();

        tokens.append_all(quote_spanned! { self.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis fn #method_name(&self) -> &[ #inner_ty ] {
                self.#field_name.as_slice()
            }
        })
    }
}

#[derive(Clone, Debug, Deref)]
struct MutGetter<'a>(&'a Getter<'a>);

impl<'a> MutGetter<'a> {
    fn as_mut_option(&'a self) -> Option<MutOptionGetter<'a>> {
        if self.is_option() {
            Some(MutOptionGetter(self))
        } else {
            None
        }
    }

    fn as_mut_slice(&'a self) -> Option<MutSliceGetter<'a>> {
        if self.is_slice() {
            Some(MutSliceGetter(self))
        } else {
            None
        }
    }

    fn method_name(&self) -> Ident {
        let prefix = self.prefix().unwrap_or_default();
        let name = self.name();

        format_ident!("{}{}_mut", prefix, name.to_string())
    }
}

impl<'a> ToTokens for MutGetter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let vis = self.vis();
        let attrs = self.field_attrs;
        let ty = &self.field.ty;
        let field_name = self.field_name();
        let method_name = self.method_name();

        tokens.append_all(quote_spanned! { self.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis fn #method_name(&mut self) -> &mut #ty {
                &mut self.#field_name
            }
        })
    }
}

#[derive(Clone, Debug, Deref)]
struct MutOptionGetter<'a>(&'a MutGetter<'a>);

impl<'a> ToTokens for MutOptionGetter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let vis = self.vis();
        let attrs = self.field_attrs;
        let inner_ty = self.option_inner_ty();
        let field_name = match self.field.ident {
            Some(ref name) => name,
            None => abort!(self.field.span(), "field should have name"),
        };
        let method_name = self.method_name();

        tokens.append_all(quote_spanned! { self.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis fn #method_name(&mut self) -> Option<&mut #inner_ty> {
                self.#field_name.as_mut()
            }
        })
    }
}

#[derive(Clone, Debug, Deref)]
struct MutSliceGetter<'a>(&'a MutGetter<'a>);

impl<'a> ToTokens for MutSliceGetter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let vis = self.vis();
        let attrs = self.field_attrs;
        let inner_ty = self.slice_inner_ty();
        let field_name = self.field_name();
        let method_name = self.method_name();

        tokens.append_all(quote_spanned! { self.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis fn #method_name(&mut self) -> &mut[ #inner_ty ] {
                self.#field_name.as_mut_slice()
            }
        })
    }
}
