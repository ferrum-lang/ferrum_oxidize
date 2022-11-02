use super::*;

use crate::parser::ast::*;

lazy_static! {
    pub static ref RUNTIME_SCOPE: ScopeTable = {
        let mut scope = ScopeTable::new();
        // TODO: Add runtime prelude pub api
        //
        // Eventually autogen bindings, and only use binding files for edge-cases
        
        scope.insert(String::from("print"), ScopeRefNode {
            name: String::from("print"),
            is_public: true,
            scope_ref: ScopeRef::Fn {
                name: String::from("print"),
                generics: None,
                params: vec![],
                return_type: None,
            },
        });

        scope
    };
}

