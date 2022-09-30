use super::*;

use std::collections::HashMap;

pub type ScopeTable = HashMap<String, ScopeRef>;

#[derive(Debug, Clone)]
pub enum ScopeRef {
    Fn {
        name: String,
        generics: Option<Vec<GenericParam>>,
        params: Vec<FnDefParamNode>,
        return_type: Option<Type>,
    }
    // Struct(StructDef),
    // ...
}

