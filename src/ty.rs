use syn::{
    AngleBracketedGenericArguments, GenericArgument, PathArguments, Type, TypeArray, TypePath,
    TypeReference,
};

pub trait TypeExt {
    fn is_ty(&self, name: &str) -> bool;

    fn is_ref_ty(&self, name: &str) -> bool;

    fn is_string(&self) -> bool;

    fn is_str(&self) -> bool;

    fn option_inner_ty(&self) -> Option<&Type>;

    fn slice_inner_ty(&self) -> Option<&Type>;
}

impl TypeExt for Type {
    fn is_ty(&self, name: &str) -> bool {
        matches!(self,
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

    fn is_ref_ty(&self, name: &str) -> bool {
        matches!(self,
           Type::Reference(TypeReference { ref elem, .. }) if elem.as_ref().is_ty(name))
    }

    fn is_string(&self) -> bool {
        self.is_ty("String")
    }

    fn is_str(&self) -> bool {
        self.is_ref_ty("str")
    }

    fn option_inner_ty(&self) -> Option<&Type> {
        inner_ty(self, "Option")
    }

    fn slice_inner_ty(&self) -> Option<&Type> {
        inner_ty(self, "Vec").or(array_elem_ty(self))
    }
}

fn inner_ty<'a>(ty: &'a Type, name: &'a str) -> Option<&'a Type> {
    generic_args_ty(ty, [name]).and_then(|args| args.into_iter().next())
}

fn array_elem_ty(ty: &Type) -> Option<&Type> {
    if let Type::Array(TypeArray { ref elem, .. }) = ty {
        Some(elem.as_ref())
    } else {
        None
    }
}

pub fn generic_args_ty<'a, I: IntoIterator<Item = &'a str>>(
    ty: &'a Type,
    names: I,
) -> Option<Vec<&'a Type>> {
    match ty {
        Type::Path(TypePath {
            ref qself,
            ref path,
        }) if qself.is_none()
            && path
                .segments
                .last()
                .map(|s| names.into_iter().any(|name| s.ident == name))
                .unwrap_or_default() =>
        {
            match path.segments.last().as_ref().unwrap().arguments {
                PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    ref args, ..
                }) => Some(
                    args.into_iter()
                        .flat_map(|arg| {
                            if let GenericArgument::Type(ty) = arg {
                                Some(ty)
                            } else {
                                None
                            }
                        })
                        .collect(),
                ),
                _ => None,
            }
        }
        _ => None,
    }
}
