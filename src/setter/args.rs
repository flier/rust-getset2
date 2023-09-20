use merge::Merge;
use structmeta::{Flag, NameArgs, NameValue, StructMeta};
use syn::{
    parse::{Parse, ParseStream},
    Ident, LitBool, LitStr, Meta, Token, Type, TypeParam,
};

use crate::{
    args::{merge_flag, merge_name_args},
    vis::Restricted,
};

#[derive(Clone, Debug, Default, Merge, StructMeta)]
pub struct StructArgs {
    #[struct_meta(name = "pub")]
    pub vis: Option<NameArgs<Option<Restricted>>>,
    #[merge(strategy = merge_flag)]
    pub into: Flag,
    #[merge(strategy = merge_flag)]
    pub try_into: Flag,
    #[merge(strategy = merge_flag)]
    pub opt: Flag,
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
    pub into: Option<NameArgs<Option<LitBool>>>,
    pub try_into: Option<NameArgs<Option<LitBool>>>,
    pub opt: Option<NameArgs<Option<LitBool>>>,
    pub extend: Option<NameArgs<Option<Extend>>>,
    pub rename: Option<NameArgs<Ident>>,
    pub prefix: Option<NameValue<LitStr>>,
    pub suffix: Option<NameValue<LitStr>>,
    #[merge(strategy = merge_name_args)]
    pub attr: Option<NameArgs<Vec<Meta>>>,
}

#[derive(Clone, Debug)]
pub enum Extend {
    Type(Type),

    Bound(TypeParam),
}

impl Parse for Extend {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Ident) && input.peek2(Token![:]) {
            input.parse().map(Extend::Bound)
        } else {
            input.parse().map(Extend::Type)
        }
    }
}
