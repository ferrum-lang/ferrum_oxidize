use super::*;

#[derive(Debug, Clone)]
pub struct LiteralNode {
    pub literal: Literal,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Bool(bool),
    String(String),
}
