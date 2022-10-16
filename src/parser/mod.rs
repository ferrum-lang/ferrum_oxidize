pub mod ast;
use ast::*;

mod error;
pub use error::ParseError;

mod parse;
pub use parse::*;

mod fill_scope;
use fill_scope::*;

use crate::lexer::token::*;
use crate::Result;

use anyhow::Context;

use ferrum_runtime::prelude::FeShared;

pub fn parse_to_ast(tokens: Vec<Token>) -> Result<FerrumFileAst> {
    let mut parser = Parser::new(tokens);
    return parse_file(&mut parser);
}

pub fn fill_project_node_scope(root_mod_node: &mut FeShared<FerrumModNode>) -> Result {
    return fill_mod_node_scope(root_mod_node);
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
            Err(e) => Err(e).with_context(|| {
                ParseError::NotExpectedToken(file!(), line!(), None, token_type)
            })?,
        }
    }

    fn consume(&mut self, token_type: TokenType) -> Result<Token> {
        let token = self.expect(token_type)?;

        self.index += 1;

        return Ok(token);
    }

    fn consume_if(&mut self, token_type: TokenType) -> Result<Option<Token>> {
        if !self.scan(&[token_type.clone()]) {
            return Ok(None);
        }

        let token = self.consume(token_type)?;
        return Ok(Some(token));
    }

    fn consume_current(&mut self) -> Result<Token> {
        let token = self.current()?;

        self.index += 1;

        return Ok(token);
    }
}
