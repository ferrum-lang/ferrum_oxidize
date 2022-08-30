mod error;
pub mod token;

pub use error::LexerError;

use token::Token;

use crate::Result;

pub fn lex_into_tokens(content: String) -> Result<Vec<Token>> {
    todo!();
}

