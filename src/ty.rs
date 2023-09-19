use syn::{
    AngleBracketedGenericArguments, GenericArgument, PathArguments, Type, TypeArray, TypePath,
    TypeReference,
};

pub trait TypeExt {
    fn is_string(&self) -> bool;

    fn is_str(&self) -> bool;

    fn option_inner_ty(&self) -> Option<Type>;

    fn slice_inner_ty(&self) -> Option<Type>;
}

impl TypeExt for Type {
    fn is_string(&self) -> bool {
        is_ty(self, "String")
    }

    fn is_str(&self) -> bool {
        is_ref_ty(self, "str")
    }

    fn option_inner_ty(&self) -> Option<Type> {
        inner_ty(self, "Option")
    }

    fn slice_inner_ty(&self) -> Option<Type> {
        inner_ty(self, "Vec").or(array_elem_ty(self))
    }
}

fn is_ty(ty: &Type, name: &str) -> bool {
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

fn is_ref_ty(ty: &Type, name: &str) -> bool {
    matches!(ty,
        Type::Reference(TypeReference { ref elem, .. }) if is_ty(elem.as_ref(), name))
}

fn inner_ty(ty: &Type, name: &str) -> Option<Type> {
    match ty {
        Type::Path(TypePath {
            ref qself,
            ref path,
        }) if qself.is_none()
            && path
                .segments
                .last()
                .map(|s| s.ident == name)
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
        _ => None,
    }
}

fn array_elem_ty(ty: &Type) -> Option<Type> {
    match ty {
        Type::Array(TypeArray { ref elem, .. }) => Some(elem.as_ref().clone()),
        _ => None,
    }
}
