use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerError {
    // #[error("Entry file not found")]
    // EntryFileNotFound,

    // #[error("Target parse error")]
    // TargetParseError(#[from] TargetParseError),
}

