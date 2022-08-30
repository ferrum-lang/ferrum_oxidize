mod error;

pub use error::ConfigError;

use crate::Target;
use crate::OxidizeError;
use crate::Result;

use std::{
    fs,
    path::PathBuf,
    str::FromStr,
};

use target_lexicon::HOST as HOST_TARGET;

pub struct Config {
    pub entry_file: Option<PathBuf>,
    pub build_dir: Option<PathBuf>,
    pub out_file: Option<PathBuf>,
    pub target: Option<String>,
}

pub fn determine_entry_file(config_entry_file: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(entry_file) = config_entry_file {
        let entry_file = fs::canonicalize(entry_file)?;
        return Ok(entry_file);
    }

    let entry_file = fs::canonicalize("./main.fe")?;
    if entry_file.exists() {
        return Ok(entry_file);
    }

    let entry_file = fs::canonicalize("./src/main.fe")?;
    if entry_file.exists() {
        return Ok(entry_file);
    }

    Err(OxidizeError::ConfigError(ConfigError::EntryFileNotFound))?;
    
    unreachable!();
}

pub fn determine_build_dir(config_build_dir: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(build_dir) = config_build_dir {
        return Ok(build_dir);
    }

    let build_dir = fs::canonicalize("./.ferrum")?;

    return Ok(build_dir);
}

pub fn determine_out_file(config_out_file: Option<PathBuf>, entry_file: &PathBuf) -> Result<PathBuf> {
    if let Some(out_file) = config_out_file {
        return Ok(out_file);
    }

    let name = entry_file.file_name()
        .expect(&format!("File name not found in entry file: {:?}", entry_file))
        .to_string_lossy()
        .to_string();

    let out_file = fs::canonicalize(name)?;

    return Ok(out_file);
}

pub fn determine_target(config_target: Option<String>) -> Result<Target> {
    let target = config_target
        .map(|value| {
            Target::from_str(&value)
                .map_err(|e| OxidizeError::ConfigError(ConfigError::TargetParseError(e)))
        })
        .unwrap_or(Ok(Target::from(HOST_TARGET)))?;

    return Ok(target);
}

