use super::*;

#[derive(Debug, Clone)]
pub struct DeclarationNode {
    pub is_const: bool,
    pub decl_token: Token,
    pub decl_pattern: DeclPatternNode,
    pub explicit_type: Option<(Token, TypeNode)>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct DeclPatternNode {
    pub decl_pattern: DeclPattern,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum DeclPattern {
    Id(Token),
    // ListDestruct(DeclPatternListDestruct),
}

