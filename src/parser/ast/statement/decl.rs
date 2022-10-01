use super::*;

#[derive(Debug, Clone)]
pub struct DeclarationNode {
    pub is_const: bool,
    pub decl_token: Token,
    pub assign_pattern: AssignPatternNode,
    pub explicit_type: Option<(Token, TypeNode)>,
    pub rhs_expr: Option<(Token, ExprNode)>,
    pub span: Span,
}

