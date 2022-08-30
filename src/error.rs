use crate::cargo::CargoError;
use crate::config::ConfigError;
use crate::generator::GeneratorError;
use crate::lexer::LexerError;
use crate::parser::ParserError;
use crate::translator::TranslatorError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum OxidizeError {
    #[error("Config error")]
    ConfigError(#[from] ConfigError),

    #[error("Lexer error")]
    LexerError(#[from] LexerError),

    #[error("Parser error")]
    ParserError(#[from] ParserError),

    #[error("Translator error")]
    TranslatorError(#[from] TranslatorError),

    #[error("Generator error")]
    GeneratorError(#[from] GeneratorError),

    #[error("Cargo error")]
    CargoError(#[from] CargoError),
}
