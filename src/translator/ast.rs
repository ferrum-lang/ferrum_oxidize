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
    pub params: Vec<FnDefParam>,
    pub return_type: Option<Type>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct FnDefParam {
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
    Literal(Literal),
}

#[derive(Debug, Clone)]
pub struct FnCall {
    pub name: String,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub enum Type {}

#[derive(Debug, Clone)]
pub enum Literal {
    Bool(bool),
    String(String),
}

