use proc_macro2::Span;
use proc_macro_error::abort;
use syn::{parse_quote, parse_quote_spanned, spanned::Spanned, Ident, ItemFn, Type};

use crate::{args, ty::TypeExt};

use super::{gen, Context};

pub fn getter(ctx: &Context) -> ItemFn {
    let mut getter = gen::getter(ctx);

    getter.sig.output = parse_quote! {
        -> &[u8]
    };
    getter.block = Box::new({
        if let Some(path) = ctx.field.args.bytes_path() {
            let ref_field_name = ctx.field.ref_name();

            parse_quote_spanned!(ctx.field.span() => {
                #path( #ref_field_name )
            })
        } else {
            let field_name = ctx.field.name();

            let method_name = if is_vec_u8(&ctx.field.ty) || is_array_u8(&ctx.field.ty) {
                "as_slice"
            } else if is_cstr_or_cstring(&ctx.field.ty) {
                "to_bytes"
            } else {
                "as_bytes"
            };

            let method = Ident::new(method_name, Span::call_site());

            parse_quote_spanned!(ctx.field.span() => {
                #field_name.#method()
            })
        }
    });

    getter
}

impl Context<'_> {
    pub fn is_bytes(&self) -> bool {
        if args::merge_bool(&self.field.args.bytes, &self.struct_args.bytes).unwrap_or_default() {
            if is_well_known_type(&self.field.ty)
                || is_vec_u8(&self.field.ty)
                || is_array_u8(&self.field.ty)
                || self.field.args.bytes_path().is_some()
            {
                return true;
            }

            if self.field.args.bytes.is_some() {
                abort!(
                    self.field.ty.span(),
                    "#[get(bytes(...))] need to specify the function that accesses the bytes"
                );
            }
        }

        false
    }
}

const WELL_KNOWN_TYPES: &[&str] = &["String", "CString", "OsString"];
const WELL_KNOWN_REF_TYPES: &[&str] = &["str", "CStr", "OsStr"];

fn is_well_known_type(ty: &Type) -> bool {
    WELL_KNOWN_TYPES
        .iter()
        .any(|name| ty.is_ty(name) || ty.is_ref_ty(name))
        || WELL_KNOWN_REF_TYPES.iter().any(|name| ty.is_ref_ty(name))
}

fn is_vec_u8(ty: &Type) -> bool {
    ty.inner_ty("Vec").map_or(false, |ty| ty.is_ty("u8"))
}

fn is_array_u8(ty: &Type) -> bool {
    ty.array_elem_ty().map_or(false, |ty| ty.is_ty("u8"))
}

fn is_cstr_or_cstring(ty: &Type) -> bool {
    ty.is_ty("CString") || ty.is_ref_ty("CString") || ty.is_ref_ty("CStr")
}
