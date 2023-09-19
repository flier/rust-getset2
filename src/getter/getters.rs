use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Field;

use crate::args;

use super::{
    BorrowExt, BorrowGetter, BorrowMutGetter, BytesExt, BytesGetter, CloneGetter, CloneableExt,
    CopyGetter, CopyableExt, FieldArgs, Getter, MutOptionGetter, MutSliceGetter, MutStrGetter,
    MutableExt, OptionExt, OptionGetter, SliceExt, SliceGetter, StrExt, StrGetter, StructArgs,
};

#[derive(Clone, Debug)]
pub struct Getters<'a> {
    pub struct_args: &'a StructArgs,
    pub field: &'a Field,
    pub field_idx: usize,
}

impl<'a> ToTokens for Getters<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let (field_args, field_attrs): (FieldArgs, _) = args::extract(&self.field.attrs, "get");

        if field_args.skip {
            return;
        }

        let getter = Getter::new(self, field_args, field_attrs.as_slice());

        if getter.is_copyable() {
            CopyGetter::from(&getter).to_tokens(tokens)
        } else if getter.is_cloneable() {
            CloneGetter::from(&getter).to_tokens(tokens)
        } else if getter.is_option() {
            OptionGetter::from(&getter).to_tokens(tokens)
        } else if getter.is_slice() {
            SliceGetter::from(&getter).to_tokens(tokens)
        } else if getter.is_str() {
            StrGetter::from(&getter).to_tokens(tokens)
        } else if getter.is_bytes() {
            BytesGetter::from(&getter).to_tokens(tokens)
        } else if getter.is_borrow() {
            BorrowGetter::from(&getter).to_tokens(tokens)
        } else {
            getter.to_tokens(tokens)
        };

        if let Some(getter) = getter.as_mutable() {
            if getter.is_option() {
                MutOptionGetter::from(&getter).to_tokens(tokens)
            } else if getter.is_slice() {
                MutSliceGetter::from(&getter).to_tokens(tokens)
            } else if getter.is_str() {
                MutStrGetter::from(&getter).to_tokens(tokens)
            } else if getter.is_borrow() {
                BorrowMutGetter::from(&getter).to_tokens(tokens)
            } else {
                getter.to_tokens(tokens)
            }
        }
    }
}
