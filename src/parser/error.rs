use super::*;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("[{0}:{1}] Unexpected end of file. The last item in the file could not be parsed.")]
    IndexOutOfBounds(&'static str, u32),

    #[error("[{0}:{1}] Unexpected token: {2:#?}")]
    UnexpectedToken(&'static str, u32, Token),

    #[error("[{0}:{1}] Unexpected token. Expected type {3:#?}, got token: {2:#?}")]
    NotExpectedToken(&'static str, u32, Option<Token>, TokenType),

    #[error("[{0}:{1}] Unexpected token. Expected new line, got token: {2:#?}")]
    NotExpectedNewline(&'static str, u32, Token),

    #[error("[{0}:{1}] Cannot use self: {2:#?}, {3:#?}")]
    CannotUseSelf(&'static str, u32, std::path::PathBuf, UseNode),

    #[error("[{0}:{1}] Use dependency not found: {2:#?}, {3:#?}")]
    UseNotFound(&'static str, u32, std::path::PathBuf, UseNode),

    #[error("[{0}:{1}] Invalid use pattern: {2:#?}")]
    InvalidUsePattern(&'static str, u32, UsePatternNode),

    #[error("[{0}:{1}] Cannot use private item: {2:#?}, {3:#?}")]
    CannotUsePrivate(&'static str, u32, UsePatternNode, ScopeRefNode),
}

