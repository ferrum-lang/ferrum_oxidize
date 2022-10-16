use super::*;

#[derive(Debug, Clone)]
pub struct StaticAccessNode {
    pub lhs: IdentLookupNode,
    pub delim: Token,
    pub rhs: Box<ExprNode>,
    pub span: Span,
}
