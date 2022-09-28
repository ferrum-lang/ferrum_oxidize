use super::*;

pub mod expr;
pub use expr::*;

#[derive(Debug, Clone)]
pub struct StatementNode {
    pub statement: Statement,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Item(Box<ItemNode>),
    Expr(ExprNode),
}

