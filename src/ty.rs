use syn::{Type, TypePath, TypeReference};

pub trait TypeExt {
    fn is_string(&self) -> bool;

    fn is_str(&self) -> bool;
}

impl TypeExt for Type {
    fn is_string(&self) -> bool {
        is_ty(self, "String")
    }

    fn is_str(&self) -> bool {
        is_ref_ty(self, "str")
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
