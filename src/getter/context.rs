use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Token, Visibility};

use crate::{args, field::Field as BaseField};

use super::{
    BorrowGetter, BorrowMutGetter, BytesGetter, CloneGetter, CopyGetter, Field, FieldArgs, Getter,
    MutGetter, MutOptionGetter, MutSliceGetter, MutStrGetter, OptionExt, OptionGetter, SliceGetter,
    StrGetter, StructArgs,
};

#[derive(Clone, Debug)]
pub struct Context<'a> {
    pub struct_args: &'a StructArgs,
    pub field: Field,
}

impl<'a> Context<'a> {
    pub fn new(struct_args: &'a StructArgs, field: BaseField) -> Self {
        let (field_args, field_attrs): (FieldArgs, _) = args::extract(field.attrs.clone(), "get");

        Self {
            struct_args,
            field: Field::new(field, field_args, field_attrs),
        }
    }

    pub fn vis(&self) -> Visibility {
        args::vis(&self.field.args.vis, &self.struct_args.vis, &self.field.vis)
    }

    pub fn constness(&self) -> Option<Token![const]> {
        args::constness(&self.field.args.constness, &self.struct_args.constness)
    }

    pub fn prefix(&self) -> Option<String> {
        args::prefix(&self.field.args.prefix, &self.struct_args.prefix)
    }

    pub fn suffix(&self) -> Option<String> {
        args::suffix(&self.field.args.suffix, &self.struct_args.suffix)
    }
}

impl<'a> ToTokens for Context<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.field.args.skip {
            return;
        }

        if self.is_copyable() {
            CopyGetter::new(self).to_tokens(tokens)
        } else if self.is_cloneable() {
            CloneGetter::new(self).to_tokens(tokens)
        } else if self.is_option() {
            OptionGetter::new(self).to_tokens(tokens)
        } else if self.is_slice() {
            SliceGetter::new(self).to_tokens(tokens)
        } else if self.is_str() {
            StrGetter::new(self).to_tokens(tokens)
        } else if self.is_bytes() {
            BytesGetter::new(self).to_tokens(tokens)
        } else if self.is_borrow() {
            BorrowGetter::new(self).to_tokens(tokens)
        } else {
            Getter::new(self).to_tokens(tokens)
        };

        if self.is_mutable() || self.is_mut_slice() || self.is_mut_str() || self.is_borrow_mut() {
            if self.is_option() {
                MutOptionGetter::new(self).to_tokens(tokens)
            } else if self.is_mut_slice() {
                MutSliceGetter::new(self).to_tokens(tokens)
            } else if self.is_mut_str() {
                MutStrGetter::new(self).to_tokens(tokens)
            } else if self.is_borrow_mut() {
                BorrowMutGetter::new(self).to_tokens(tokens)
            } else {
                MutGetter::new(self).to_tokens(tokens)
            }
        }
    }
}
