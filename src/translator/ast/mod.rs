mod expr;
pub use expr::*;

mod fn_def;
pub use fn_def::*;

mod stmt;
pub use stmt::*;

mod r#type;
pub use r#type::*;

mod r#use;
pub use r#use::*;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RustProject {
    pub main_file: RustFileAst,
    pub siblings: HashMap<String, RustModNode>,
}

#[derive(Debug, Clone)]
pub struct RustModNode {
    pub name: String,
    pub file: RustModNodeFile,
}

#[derive(Debug, Clone)]
pub enum RustModNodeFile {
    File(RustFileAst),
    Dir(HashMap<String, RustModNode>),
}

#[derive(Debug, Clone)]
pub struct RustFileAst {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub enum Item {
    Use(Use),
    Mod(Mod),
    FnDef(FnDef),
}

