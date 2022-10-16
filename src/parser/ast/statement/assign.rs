use super::*;

#[derive(Debug, Clone)]
pub struct AssignNode {
    pub lhs: Box<StatementNode>,
    pub rhs: ExprNode,
    pub assign: Token,
    pub span: Span,
}

