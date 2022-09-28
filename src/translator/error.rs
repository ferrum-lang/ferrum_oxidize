use super::*;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TranslateError {
    #[error("[{0}:{1}] Invalid top-level statement: {2:#?}\nNote: Top-level statements are only allowed in entry files, and only when no main function exists.\n")]
    InvalidTopLevelStatement(&'static str, u32, parser::ast::StatementNode),

    // #[error("Entry file not found")]
    // EntryFileNotFound,

    // #[error("Target parse error")]
    // TargetParseError(#[from] TargetParseError),
}

