use merge::Merge;
use structmeta::{Flag, NameArgs, NameValue, StructMeta};
use syn::{Ident, LitBool, LitStr, Path, Type};

use crate::vis::Restricted;

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
    pub slice: Option<NameArgs<Option<Path>>>,
    pub mut_slice: Option<NameArgs<Option<Path>>>,
    pub str: Option<NameArgs<Option<Path>>>,
    pub mut_str: Option<NameArgs<Option<Path>>>,
    pub bytes: Option<NameArgs<Option<Path>>>,
    pub borrow: Option<NameArgs<Type>>,
    pub borrow_mut: Option<NameArgs<Type>>,
    pub rename: Option<NameArgs<Ident>>,
    pub prefix: Option<NameValue<LitStr>>,
    pub suffix: Option<NameValue<LitStr>>,
}

fn merge_flag(lhs: &mut Flag, rhs: Flag) {
    if rhs.span.is_some() {
        lhs.span = rhs.span
    }
}
