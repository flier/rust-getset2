use syn::{
    AngleBracketedGenericArguments, GenericArgument, PathArguments, Type, TypeArray, TypePath,
    TypeReference,
};

pub trait TypeExt {
    fn is_ty(&self, name: &str) -> bool;

    fn is_ref_ty(&self, name: &str) -> bool;

    fn is_string(&self) -> bool;

    fn is_ref_string(&self) -> bool;

    fn is_str(&self) -> bool;

    fn ref_elem_ty(&self) -> Option<&Type>;

    fn array_elem_ty(&self) -> Option<&Type>;

    fn option_inner_ty(&self) -> Option<&Type>;

    fn slice_inner_ty(&self) -> Option<&Type>;

    fn inner_ty(&self, name: &str) -> Option<&Type>;
}

impl TypeExt for Type {
    fn is_str(&self) -> bool {
        self.is_ref_ty("str")
    }

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

    fn is_string(&self) -> bool {
        self.is_ty("String")
    }

    fn is_ref_string(&self) -> bool {
        self.is_ref_ty("String")
    }

    fn is_ref_ty(&self, name: &str) -> bool {
        matches!(self,
           Type::Reference(TypeReference { ref elem, .. }) if elem.as_ref().is_ty(name))
    }

    fn ref_elem_ty(&self) -> Option<&Type> {
        if let Type::Reference(TypeReference { ref elem, .. }) = self {
            Some(elem.as_ref())
        } else {
            None
        }
    }

    fn array_elem_ty(&self) -> Option<&Type> {
        if let Type::Array(TypeArray { ref elem, .. }) = self {
            Some(elem.as_ref())
        } else {
            None
        }
    }

    fn option_inner_ty(&self) -> Option<&Type> {
        self.inner_ty("Option")
    }

    fn slice_inner_ty(&self) -> Option<&Type> {
        self.inner_ty("Vec").or(self.array_elem_ty())
    }

    fn inner_ty(&self, name: &str) -> Option<&Type> {
        generic_args_ty(self, [name]).and_then(|args| args.into_iter().next())
    }
}

pub fn generic_args_ty<I: IntoIterator<Item = S>, S: AsRef<str>>(
    ty: &Type,
    names: I,
) -> Option<Vec<&Type>> {
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
