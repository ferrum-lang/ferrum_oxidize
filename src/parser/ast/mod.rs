/*

files:
/src
|- /utils
|  |- mod.fe
|  |- string.fe
|- main.fe
|- other.fe

-->

Project {
    root: Node {
        file: /src/main.fe
        mods: {
            "utils": Node {
                file: /src/utils/mod.fe
                mods: {
                    "string": Node {
                        file: /src/utils/string.fe
                        mods: {}
                    }
                }
            },
            "other": Node {
                file: /src/other.fe
                mods: {}
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

pub use crate::span::Span;
pub use crate::lexer::token::{Token, TokenType};
pub use crate::punctuated::Punctuated;

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FerrumProjectAst {
    pub root: FerrumProjectAstNode,
}

#[derive(Debug, Clone)]
pub struct FerrumProjectAstNode {
    pub file: FerrumFileAst,
    pub nodes: Vec<FerrumProjectAstNode>,
}

#[derive(Debug, Clone)]
pub struct FerrumFileAst {
    pub name: String,
    pub path: PathBuf,
    pub items: Vec<ItemNode>,
    pub scope: ScopeTable,
    pub is_mod_root: bool,
    pub span: Span,
}

impl FerrumFileAst {
    pub fn new(name: String, path: PathBuf, is_mod_root: bool) -> Self {
        return Self {
            name,
            path,
            is_mod_root,
            items: vec![],
            scope: ScopeTable::new(),
            span: Span { from: (0, 0).into(), to: (0, 0).into() }
        };
    }
}

