use merge::Merge;
use structmeta::{NameArgs, NameValue, StructMeta};
use syn::{Ident, LitStr};

use crate::vis::Restricted;

#[derive(Clone, Debug, Default, Merge, StructMeta)]
pub struct StructArgs {
    #[struct_meta(name = "pub")]
    pub vis: Option<NameArgs<Option<Restricted>>>,
    pub prefix: Option<NameValue<LitStr>>,
    pub suffix: Option<NameValue<LitStr>>,
}

#[derive(Clone, Debug, Default, Merge, StructMeta)]
pub struct FieldArgs {
    #[struct_meta(name = "pub")]
    pub vis: Option<NameArgs<Option<Restricted>>>,
    #[merge(strategy = merge::bool::overwrite_false)]
    pub skip: bool,
    pub rename: Option<NameArgs<Ident>>,
    pub prefix: Option<NameValue<LitStr>>,
    pub suffix: Option<NameValue<LitStr>>,
}
