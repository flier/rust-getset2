use derive_more::Deref;
use merge::Merge;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use proc_macro_error::abort;
use quote::{format_ident, quote_spanned, ToTokens, TokenStreamExt};
use structmeta::{Flag, NameArgs, NameValue, StructMeta};
use syn::{
    parse_quote_spanned, spanned::Spanned, Attribute, Data, DataStruct, DeriveInput, Field, Fields,
    FieldsNamed, FieldsUnnamed, Index, LitBool, LitStr, Member, Type, Visibility,
};

use crate::{extract, vis::Restricted};

#[derive(Clone, Debug, Default, Merge, StructMeta)]
struct StructArgs {
    #[struct_meta(name = "pub")]
    public: Option<NameArgs<Option<Restricted>>>,
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
    prefix: Option<NameValue<LitStr>>,
    suffix: Option<NameValue<LitStr>>,
}

#[derive(Clone, Debug, Default, Merge, StructMeta)]
struct FieldArgs {
    #[struct_meta(name = "pub")]
    public: Option<NameArgs<Option<Restricted>>>,
    #[merge(strategy = merge::bool::overwrite_false)]
    skip: bool,
    clone: Option<NameArgs<Option<LitBool>>>,
    copy: Option<NameArgs<Option<LitBool>>>,
    #[struct_meta(name = "mut")]
    mutable: Option<NameArgs<Option<LitBool>>>,
    #[struct_meta(name = "opt")]
    option: Option<NameArgs<Option<LitBool>>>,
    slice: Option<NameArgs<Option<LitBool>>>,
    rename: Option<LitStr>,
    prefix: Option<NameValue<LitStr>>,
    suffix: Option<NameValue<LitStr>>,
}

fn merge_flag(lhs: &mut Flag, rhs: Flag) {
    if rhs.span.is_some() {
        lhs.span = rhs.span
    }
}

pub fn expand(input: DeriveInput) -> syn::Result<TokenStream2> {
    let (struct_args, _): (StructArgs, _) = extract::args(&input.attrs, "get");

    if let Data::Struct(DataStruct { fields, .. }) = &input.data {
        let name = &input.ident;
        let generics = &input.generics;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let fields = match fields {
            Fields::Named(FieldsNamed { named, .. }) => named,
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => unnamed,
            Fields::Unit => {
                abort!(
                    input,
                    "#[derive(Getter)] can only be applied to structure with fields"
                )
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

        Ok(quote_spanned! { input.span() =>
            impl #impl_generics #name #ty_generics #where_clause {
                #( #getters )*
            }
        })
    } else {
        abort!(input, "#[derive(Getter)] can only be applied to structure")
    }
}

#[derive(Clone, Debug)]
struct Getters<'a> {
    struct_args: &'a StructArgs,
    field: &'a Field,
    field_idx: usize,
}

impl<'a> ToTokens for Getters<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let (field_args, field_attrs): (FieldArgs, _) = extract::args(&self.field.attrs, "get");

        if field_args.skip {
            return;
        }

        let getter = Getter {
            getters: self,
            field_args,
            field_attrs: field_attrs.as_slice(),
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

#[derive(Clone, Debug, Deref)]
struct Getter<'a> {
    #[deref]
    getters: &'a Getters<'a>,
    field_args: FieldArgs,
    field_attrs: &'a [&'a Attribute],
}

impl<'a> Getter<'a> {
    fn as_copyable(&'a self) -> Option<CopyGetter<'a>> {
        self.field_args
            .copy
            .as_bool()
            .or(self.struct_args.copy.as_bool())
            .and_then(|b| if b { Some(CopyGetter(self)) } else { None })
    }

    fn as_cloneable(&'a self) -> Option<CloneGetter<'a>> {
        self.field_args
            .clone
            .as_bool()
            .or(self.struct_args.clone.as_bool())
            .and_then(|b| if b { Some(CloneGetter(self)) } else { None })
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
        self.field_args
            .mutable
            .as_bool()
            .or(self.struct_args.mutable.as_bool())
            .and_then(|b| if b { Some(MutGetter(self)) } else { None })
    }

    fn vis(&self) -> Visibility {
        if let Some(arg) = self.field_args.public.as_ref() {
            if let Some(ref r) = arg.args {
                return r.clone().into();
            } else {
                return parse_quote_spanned! { self.field.span() => pub };
            }
        }

        if let Some(arg) = self.struct_args.public.as_ref() {
            if let Some(ref r) = arg.args {
                return r.clone().into();
            } else {
                return parse_quote_spanned! { self.field.span() => pub };
            }
        }

        return self.field.vis.clone();
    }

    fn field_name(&self) -> TokenStream2 {
        match self.field.ident {
            Some(ref name) => quote_spanned! { self.field.span() => self.#name },
            None => {
                let idx = Member::Unnamed(Index {
                    index: self.field_idx as u32,
                    span: self.field.span(),
                });

                quote_spanned! { self.field.span() => self.#idx }
            }
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
            .map(|s| format!("{}_", s.value.value()))
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

        rename.unwrap_or_else(|| match self.field.ident {
            Some(ref name) => name.clone(),
            None => format_ident!("arg{}", self.field_idx),
        })
    }

    fn is_option(&self) -> bool {
        if self
            .field_args
            .option
            .as_bool()
            .or(self.struct_args.option.as_bool())
            .unwrap_or_default()
        {
            if extract::option_inner_ty(&self.field.ty).is_some() {
                return true;
            }

            if self.field_args.option.as_bool().unwrap_or_default() {
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
        if self
            .field_args
            .slice
            .as_bool()
            .or(self.struct_args.slice.as_bool())
            .unwrap_or_default()
        {
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
                & #field_name
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
                #field_name
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
                #field_name.clone()
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
                #field_name.as_ref()
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
                #field_name.as_slice()
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
                &mut #field_name
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
        let field_name = self.field_name();
        let method_name = self.method_name();

        tokens.append_all(quote_spanned! { self.field.span() =>
            #( #attrs )*
            #[inline(always)]
            #vis fn #method_name(&mut self) -> Option<&mut #inner_ty> {
                #field_name.as_mut()
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
                #field_name.as_mut_slice()
            }
        })
    }
}
