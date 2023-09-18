use merge::Merge;
use proc_macro_error::abort;
use syn::{
    parse::Parse, spanned::Spanned, AngleBracketedGenericArguments, AttrStyle, Attribute,
    GenericArgument, PathArguments, Type, TypeArray, TypePath, TypeReference,
};

pub fn args<'a, I, T>(attrs: I, name: &str) -> (T, Vec<&'a Attribute>)
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
        .filter(|attr| attr.style == AttrStyle::Outer)
        .collect::<Vec<_>>();

    (args, attrs)
}

pub fn option_inner_ty(ty: &Type) -> Option<Type> {
    match ty {
        Type::Path(TypePath {
            ref qself,
            ref path,
        }) if qself.is_none()
            && path
                .segments
                .last()
                .map(|s| s.ident == "Option")
                .unwrap_or_default() =>
        {
            match path.segments.last().cloned().unwrap().arguments {
                PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. })
                    if args.len() == 1 =>
                {
                    match args.first() {
                        Some(GenericArgument::Type(ty)) => return Some(ty.clone()),
                        _ => None,
                    }
                }
                _ => None,
            }
        }
        _ => None,
    }
}

pub fn slice_inner_ty(ty: &Type) -> Option<Type> {
    match ty {
        Type::Path(TypePath {
            ref qself,
            ref path,
        }) if qself.is_none()
            && path
                .segments
                .last()
                .map(|s| s.ident == "Vec")
                .unwrap_or_default() =>
        {
            match path.segments.last().cloned().unwrap().arguments {
                PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. })
                    if args.len() == 1 =>
                {
                    match args.first() {
                        Some(GenericArgument::Type(ty)) => Some(ty.clone()),
                        _ => None,
                    }
                }
                _ => None,
            }
        }
        Type::Array(TypeArray { ref elem, .. }) => Some(elem.as_ref().clone()),
        _ => None,
    }
}

pub fn is_string_ty(ty: &Type) -> bool {
    is_ty(ty, "String")
}

pub fn is_str_ty(ty: &Type) -> bool {
    is_ref_ty(ty, "str")
}
pub fn is_cstring_ty(ty: &Type) -> bool {
    is_ty(ty, "CString")
}

pub fn is_cstr_ty(ty: &Type) -> bool {
    is_ref_ty(ty, "CStr")
}

pub fn is_ty(ty: &Type, name: &str) -> bool {
    matches!(ty,
        Type::Path(TypePath {
            ref qself,
            ref path,
        }) if qself.is_none()
            && path
                .segments
                .last()
                .map(|s| s.ident == name)
                .unwrap_or_default())
}

pub fn is_ref_ty(ty: &Type, name: &str) -> bool {
    matches!(ty,
        Type::Reference(TypeReference { elem, .. }) if is_ty(elem.as_ref(), name))
}
