use super::*;

#[derive(Debug, Clone)]
pub struct GenericTypeNode {
    pub generic_type: GenericType,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum GenericType {}
