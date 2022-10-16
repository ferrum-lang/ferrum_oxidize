use super::*;

pub mod expr;
pub use expr::*;

pub mod assign;
pub use assign::*;

pub mod decl;
pub use decl::*;

#[derive(Debug, Clone)]
pub struct StatementNode {
    pub statement: Statement,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expr(ExprNode),
    Decl(DeclarationNode),
    Assign(AssignNode),
}

