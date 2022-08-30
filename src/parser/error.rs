use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    // #[error("Entry file not found")]
    // EntryFileNotFound,

    // #[error("Target parse error")]
    // TargetParseError(#[from] TargetParseError),
}

