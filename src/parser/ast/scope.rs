use super::*;

use std::collections::HashMap;

pub type ScopeTable = HashMap<String, ScopeRefNode>;

#[derive(Debug, Clone)]
pub struct ScopeRefNode {
    pub name: String,
    pub is_public: bool,
    pub scope_ref: ScopeRef,
}

#[derive(Debug, Clone)]
pub enum ScopeRef {
    Mod(ScopeTable),
    Fn {
        name: String,
        generics: Option<Vec<GenericParam>>,
        params: Vec<FnDefParamNode>,
        return_type: Option<Type>,
    },
    LocalVar {
        is_const: bool,
        name: String,
        known_type: Option<Type>,
    },
    // Struct(StructDef),
    // ...
}

