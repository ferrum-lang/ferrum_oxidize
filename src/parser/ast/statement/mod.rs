use super::*;

pub mod expr;
pub use expr::*;

pub mod assign;
pub use assign::*;

pub mod decl;
pub use decl::*;

pub mod r#do;
pub use r#do::*;

#[derive(Debug, Clone)]
pub struct StatementNode {
    pub statement: Statement,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Pass,
    Expr(ExprNode),
    Decl(DeclarationNode),
    Assign(AssignNode),
    Do(DoNode),
}

