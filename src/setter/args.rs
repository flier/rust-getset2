use merge::Merge;
use structmeta::{Flag, NameArgs, NameValue, StructMeta};
use syn::{Ident, LitBool, LitStr};

use crate::{args::merge_flag, vis::Restricted};

#[derive(Clone, Debug, Default, Merge, StructMeta)]
pub struct StructArgs {
    #[struct_meta(name = "pub")]
    pub vis: Option<NameArgs<Option<Restricted>>>,
    #[merge(strategy = merge_flag)]
    pub into: Flag,
    pub prefix: Option<NameValue<LitStr>>,
    pub suffix: Option<NameValue<LitStr>>,
}

#[derive(Clone, Debug, Default, Merge, StructMeta)]
pub struct FieldArgs {
    #[struct_meta(name = "pub")]
    pub vis: Option<NameArgs<Option<Restricted>>>,
    #[merge(strategy = merge::bool::overwrite_false)]
    pub skip: bool,
    pub into: Option<NameArgs<Option<LitBool>>>,
    pub rename: Option<NameArgs<Ident>>,
    pub prefix: Option<NameValue<LitStr>>,
    pub suffix: Option<NameValue<LitStr>>,
}
