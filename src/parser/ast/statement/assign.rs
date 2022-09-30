use super::*;

#[derive(Debug, Clone)]
pub struct AssignNode {
    pub is_const: bool,
    pub assign_token: Token,
    pub name: Token,
    pub explicit_type: Option<(Token, TypeNode)>,
    pub rhs_expr: Option<(Token, ExprNode)>,
    pub span: Span,
}

