use crate::cargo::CargoError;
use crate::config::ConfigError;
use crate::generator::GeneratorError;
use crate::lexer::LexerError;
use crate::parser::ParseError;
use crate::translator::TranslateError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum OxidizeError {
    #[error("Config error")]
    ConfigError(#[from] ConfigError),

    #[error("Lexer error")]
    LexerError(#[from] LexerError),

    #[error("Parse error")]
    ParserError(#[from] ParseError),

    #[error("Translate error")]
    TranslatorError(#[from] TranslateError),

    #[error("Generator error")]
    GeneratorError(#[from] GeneratorError),

    #[error("Cargo error")]
    CargoError(#[from] CargoError),
}
