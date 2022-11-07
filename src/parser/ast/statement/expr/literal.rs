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
    SomeOption {
        token: Token,
        open_paren: Token,
        expr: Box<ExprNode>,
        close_paren: Token,
    },
    NoneOption(Token),
}
