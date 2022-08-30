pub mod ast;
mod error;

use ast::FerrumFileAst;

pub use error::ParserError;

use crate::lexer::token::Token;
use crate::Result;

pub fn parse_to_ast(tokens: Vec<Token>) -> Result<FerrumFileAst> {
    todo!();
}

