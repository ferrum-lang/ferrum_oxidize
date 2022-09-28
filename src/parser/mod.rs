pub mod ast;
use anyhow::Context;
use ast::*;

mod error;
pub use error::ParseError;

mod parse;
pub use parse::*;

use crate::lexer::token::{Token, TokenType};
use crate::Result;

pub fn parse_to_ast(tokens: Vec<Token>) -> Result<FerrumFileAst> {
    let mut parser = Parser::new(tokens);
    return parse_file(&mut parser);
}

pub struct Parser {
    pub tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        return Self { tokens, index: 0 };
    }

    fn current(&self) -> Result<Token> {
        if self.index >= self.tokens.len() {
            Err(ParseError::IndexOutOfBounds(file!(), line!()))?;
        }

        return Ok(self.tokens[self.index].clone());
    }

    fn next(&self) -> Result<Token> {
        if self.index + 1 >= self.tokens.len() {
            Err(ParseError::IndexOutOfBounds(file!(), line!()))?;
        }

        return Ok(self.tokens[self.index + 1].clone());
    }

    fn scan(&self, token_types: &[TokenType]) -> bool {
        let mut index: usize = self.index;

        for i in 0..token_types.len() {
            if index >= self.tokens.len() {
                return false;
            }

            if self.tokens[index].token_type != token_types[i] {
                return false;
            }

            index += 1;
        }

        return true;
    }

    fn expect(&self, token_type: TokenType) -> Result<Token> {
        match self.current() {
            Ok(token) if token.token_type == token_type => return Ok(token),
            Ok(token) => Err(ParseError::NotExpectedToken(
                file!(),
                line!(),
                Some(token),
                token_type,
            ))?,
            Err(e) => Err(e).with_context(|| ParseError::NotExpectedToken(file!(), line!(), None, token_type))?,
        }
    }

    fn consume(&mut self, token_type: TokenType) -> Result<Token> {
        let token = self.expect(token_type)?;

        self.index += 1;

        return Ok(token);
    }

    fn consume_if(&mut self, token_type: TokenType) -> Option<Token> {
        if !self.scan(&[token_type.clone()]) {
            return None;
        }

        let token = self.consume(token_type).unwrap();
        return Some(token);
    }
}
