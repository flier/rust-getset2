use merge::Merge;
use structmeta::{Flag, NameArgs, NameValue, StructMeta};
use syn::{ExprPath, Ident, LitBool, LitStr, Meta, Type};

use crate::{
    args::{merge_flag, merge_name_args},
    vis::Restricted,
};

#[derive(Clone, Debug, Default, Merge, StructMeta)]
pub struct StructArgs {
    #[struct_meta(name = "pub")]
    pub vis: Option<NameArgs<Option<Restricted>>>,
    #[struct_meta(name = "const")]
    #[merge(strategy = merge_flag)]
    pub constness: Flag,
    #[merge(strategy = merge_flag)]
    pub clone: Flag,
    #[merge(strategy = merge_flag)]
    pub copy: Flag,
    #[struct_meta(name = "mut")]
    #[merge(strategy = merge_flag)]
    pub mutable: Flag,
    #[merge(strategy = merge_flag)]
    pub opt: Flag,
    #[merge(strategy = merge_flag)]
    pub slice: Flag,
    #[merge(strategy = merge_flag)]
    pub mut_slice: Flag,
    #[merge(strategy = merge_flag)]
    pub str: Flag,
    #[merge(strategy = merge_flag)]
    pub mut_str: Flag,
    #[merge(strategy = merge_flag)]
    pub bytes: Flag,
    pub prefix: Option<NameValue<LitStr>>,
    pub suffix: Option<NameValue<LitStr>>,
    #[merge(strategy = merge_name_args)]
    pub attrs: Option<NameArgs<Vec<LitStr>>>,
}

impl StructArgs {
    pub fn allowed_attrs(&self) -> Option<Vec<String>> {
        self.attrs
            .as_ref()
            .map(|arg| arg.args.iter().map(|s| s.value()).collect())
    }
}

#[derive(Clone, Debug, Default, Merge, StructMeta)]
pub struct FieldArgs {
    #[struct_meta(name = "pub")]
    pub vis: Option<NameArgs<Option<Restricted>>>,
    #[merge(strategy = merge::bool::overwrite_false)]
    pub skip: bool,
    #[struct_meta(name = "const")]
    pub constness: Option<NameArgs<Option<LitBool>>>,
    pub clone: Option<NameArgs<Option<LitBool>>>,
    pub copy: Option<NameArgs<Option<LitBool>>>,
    #[struct_meta(name = "mut")]
    pub mutable: Option<NameArgs<Option<LitBool>>>,
    pub opt: Option<NameArgs<Option<LitBool>>>,
    pub slice: Option<NameArgs<Option<ExprPath>>>,
    pub mut_slice: Option<NameArgs<Option<ExprPath>>>,
    pub str: Option<NameArgs<Option<ExprPath>>>,
    pub mut_str: Option<NameArgs<Option<ExprPath>>>,
    pub bytes: Option<NameArgs<Option<ExprPath>>>,
    pub borrow: Option<NameArgs<Type>>,
    pub borrow_mut: Option<NameArgs<Type>>,
    pub rename: Option<NameArgs<Ident>>,
    pub prefix: Option<NameValue<LitStr>>,
    pub suffix: Option<NameValue<LitStr>>,
    #[merge(strategy = merge_name_args)]
    pub attr: Option<NameArgs<Vec<Meta>>>,
}

impl FieldArgs {
    pub fn borrow_ty(&self) -> Option<&Type> {
        self.borrow.as_ref().map(|arg| &arg.args)
    }

    pub fn borrow_mut_ty(&self) -> Option<&Type> {
        self.borrow_mut.as_ref().map(|arg| &arg.args)
    }

    pub fn slice_path(&self) -> Option<&ExprPath> {
        extract_path(&self.slice)
    }

    pub fn mut_slice_path(&self) -> Option<&ExprPath> {
        extract_path(&self.mut_slice)
    }

    pub fn str_path(&self) -> Option<&ExprPath> {
        extract_path(&self.str)
    }

    pub fn mut_str_path(&self) -> Option<&ExprPath> {
        extract_path(&self.mut_str)
    }

    pub fn bytes_path(&self) -> Option<&ExprPath> {
        extract_path(&self.bytes)
    }
}

fn extract_path(p: &Option<NameArgs<Option<ExprPath>>>) -> Option<&ExprPath> {
    p.as_ref().and_then(|arg| arg.args.as_ref())
}
