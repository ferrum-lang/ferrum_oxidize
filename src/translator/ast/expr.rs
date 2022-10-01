use super::*;

#[derive(Debug, Clone)]
pub enum Expr {
    FnCall(FnCall),
    Literal(Literal),
    IdentLookup(String),
    SharedRef(Box<Expr>),
    MutRef(Box<Expr>),
}

#[derive(Debug, Clone)]
pub struct FnCall {
    pub name: String,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Bool(bool),
    String(String),
}

