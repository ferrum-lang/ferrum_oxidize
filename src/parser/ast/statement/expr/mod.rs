use super::*;

pub mod ident_lookup;
pub use ident_lookup::*;

pub mod fn_call;
pub use fn_call::*;

pub mod literal;
pub use literal::*;

pub mod static_access;
pub use static_access::*;

#[derive(Debug, Clone)]
pub struct ExprNode {
    pub expr: Expr,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Expr {
    IdentLookup(IdentLookupNode),
    StaticAccess(StaticAccessNode),
    FnCall(FnCallNode),
    Literal(LiteralNode),
    Ref(RefNode),
    Deref(DerefNode),
    TemplateString(TemplateStringNode),
}

#[derive(Debug, Clone)]
pub struct RefNode {
    pub ref_token: Token,
    pub mut_token: Option<Token>,
    pub expr: Box<ExprNode>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct DerefNode {
    pub deref_token: Token,
    pub expr: Box<ExprNode>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct TemplateStringNode {
    pub start_value: String,
    pub middles: Vec<TemplateStringMiddleNode>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct TemplateStringMiddleNode {
    pub expr: Box<ExprNode>,
    pub post_value: String,
    pub span: Span,
}

