use super::*;

pub mod ident_lookup;
pub use ident_lookup::*;

pub mod fn_call;
pub use fn_call::*;

#[derive(Debug, Clone)]
pub struct ExprNode {
    pub expr: Expr,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Expr {
    IdentLookup(IdentLookupNode),
    FnCall(FnCallNode),
}

