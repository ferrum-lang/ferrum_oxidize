use super::*;

pub mod expr;
pub use expr::*;

pub mod assign;
pub use assign::*;

#[derive(Debug, Clone)]
pub struct StatementNode {
    pub statement: Statement,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expr(ExprNode),
    Assign(AssignNode),
}

