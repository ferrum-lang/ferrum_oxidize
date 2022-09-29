mod expr;
pub use expr::*;

mod fn_def;
pub use fn_def::*;

mod stmt;
pub use stmt::*;

mod r#type;
pub use r#type::*;

#[derive(Debug, Clone)]
pub struct RustProjectAst {
    pub root: RustProjectAstNode,
}

#[derive(Debug, Clone)]
pub struct RustProjectAstNode {
    pub file: RustFileAst,
    pub nodes: Vec<RustProjectAstNode>,
}

#[derive(Debug, Clone)]
pub struct RustFileAst {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub enum Item {
    FnDef(FnDef),
}

