use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeneratorError {
    // #[error("Entry file not found")]
    // EntryFileNotFound,

    // #[error("Target parse error")]
    // TargetParseError(#[from] TargetParseError),
}



