/*

files:
/src
|- /utils
|  |- mod.rs
|  |- string.rs
|- lib.rs
|- other.rs

-->

Project {
    root: Node {
        file: /src/lib.rs
        nodes: [
            Node {
                file: /src/utils/mod.rs
                nodes: [
                    Node {
                        file: /src/utils/string.rs
                        nodes: []
                    }
                ]
            },
            Node {
                file: /src/other.rs
                nodes: []
            }
        ]
    }
}

*/

pub mod statement;
pub use statement::*;

pub mod item;
pub use item::*;

pub mod fn_def;
pub use fn_def::*;

pub mod r#type;
pub use r#type::*;

pub use crate::span::Span;
pub use crate::lexer::token::{Token, TokenType};
pub use crate::punctuated::Punctuated;

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
    pub items: Vec<ItemNode>,
    pub span: Span,
}

impl FerrumFileAst {
    pub fn new() -> Self {
        return Self {
            items: vec![],
            span: Span { from: (0, 0).into(), to: (0, 0).into() }
        };
    }
}

