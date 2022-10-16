/*

files:
/src
|- /utils
|  |- _pkg.fe
|  |- string.fe
|- _main.fe
|- other.fe

-->

Project {
    root: Node {
        file: /src/_main.fe
        nodes: {
            "utils": Node {
                file: /src/utils/_pkg.fe
                nodes: {
                    "string": Node {
                        file: /src/utils/string.fe
                        nodes: {}
                    }
                }
            },
            "other": Node {
                file: /src/other.fe
                nodes: {}
            }
        }
    }
}

*/

pub mod statement;
pub use statement::*;

pub mod item;
pub use item::*;

pub mod fn_def;
pub use fn_def::*;

pub mod generics;
pub use generics::*;

pub mod scope;
pub use scope::*;

pub mod r#type;
pub use r#type::*;

pub mod r#use;
pub use r#use::*;

pub use crate::lexer::token::{Token, TokenType};
pub use crate::punctuated::Punctuated;
pub use crate::span::Span;

use std::collections::HashMap;
use std::path::PathBuf;

use ferrum_runtime::prelude::FeShared;

static mut CURRENT_ID: usize = 1;

pub fn uuid() -> usize {
    let id = unsafe { CURRENT_ID };
    unsafe { CURRENT_ID += 1 };
    return id;
}

pub struct FerrumModNode {
    pub id: usize,
    pub name: String,
    pub path: PathBuf,
    pub file: FerrumModNodeFile,
    pub sibling_refs: HashMap<String, FeShared<FerrumModNode>>,
    pub parent_ref: Option<Box<FeShared<FerrumModNode>>>,
}

impl FerrumModNode {
    pub fn new(name: String, path: PathBuf, file: FerrumModNodeFile) -> Self {
        return Self {
            id: uuid(),
            name,
            path,
            file,
            sibling_refs: HashMap::new(),
            parent_ref: None,
        };
    }
}

impl Clone for FerrumModNode {
    fn clone(&self) -> Self {
        let mut new = Self {
            id: self.id.clone(),
            name: self.name.clone(),
            path: self.path.clone(),
            file: self.file.clone(),
            sibling_refs: HashMap::new(),
            parent_ref: None,
        };

        for (name, sibling_ref) in self.sibling_refs.iter() {
            if name.as_str() != self.name.as_str() {
                new.sibling_refs
                    .insert(name.clone(), FeShared::share(sibling_ref));
            }
        }

        if let Some(parent_ref) = &self.parent_ref {
            new.parent_ref = Some(Box::new(FeShared::share(parent_ref)));
        }

        return new;
    }
}

impl std::fmt::Debug for FerrumModNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut sibling_refs = HashMap::new();

        for (name, sibling_ref) in &self.sibling_refs {
            let mut clone = sibling_ref.clone();
            clone.sibling_refs = HashMap::new();
            sibling_refs.insert(name, clone);
        }

        return write!(
            f,
            "FerrumModNode {{\n    id: {:#?},\n    name: {:#?},\n    path: {:#?},\n    file: {:#?},\n    sibling_refs: {:#?}\n}}",
            self.id, self.name, self.path, self.file, sibling_refs,
        );
    }
}

#[derive(Debug, Clone)]
pub enum FerrumModNodeFile {
    File(FerrumFileAst),
    Dir(HashMap<String, FeShared<FerrumModNode>>),
}

#[derive(Debug, Clone)]
pub struct FerrumFileAst {
    pub items: Vec<FeShared<ItemNode>>,
    pub pub_api: ScopeTable,
    pub scope: ScopeTable,
    pub span: Span,
}

impl FerrumFileAst {
    pub fn new() -> Self {
        return Self {
            items: vec![],
            pub_api: ScopeTable::new(),
            scope: ScopeTable::new(),
            span: Span {
                from: (0, 0).into(),
                to: (0, 0).into(),
            },
        };
    }
}
