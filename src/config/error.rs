use thiserror::Error;

use crate::target::TargetParseError;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Entry file not found")]
    EntryFileNotFound,

    #[error("Target parse error")]
    TargetParseError(#[from] TargetParseError),
}

