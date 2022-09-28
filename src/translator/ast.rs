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

#[derive(Debug, Clone)]
pub struct FnDef {
    pub name: String,
    pub params: Vec<FnParam>,
    pub return_type: Option<Type>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct FnParam {
    pub name: String,
    pub param_type: Type,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Item(Item),
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    FnCall(FnCall),
}

#[derive(Debug, Clone)]
pub struct FnCall {
    pub name: String,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub enum Type {}

