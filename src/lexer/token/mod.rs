mod keyword;
mod literal;
mod primitive;
mod token_type;

pub use keyword::Keyword as TokenKeyword;
pub use literal::Literal as TokenLiteral;
pub use primitive::Primitive as TokenPrimitive;
pub use token_type::TokenType;

use crate::span::Span;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub span: Span,
}

impl Token {
    pub fn new(token_type: TokenType, literal: impl Into<String>, span: impl Into<Span>) -> Self {
        return Self {
            token_type,
            literal: literal.into(),
            span: span.into(),
        };
    }
}

