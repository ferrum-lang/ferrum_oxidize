use super::*;

#[derive(Debug, Clone)]
pub struct StaticAccessNode {
    pub lhs: Box<ExprNode>,
    pub delim: Token,
    pub rhs: Box<ExprNode>,
    pub span: Span,
}
