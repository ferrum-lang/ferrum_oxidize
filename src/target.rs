use std::{
    fmt,
    str::FromStr,
};

use target_lexicon::{
    ParseError as TripleParseError,
    Triple,
};

use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Target {
    value: Triple,
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.value);
    }
}

impl From<Triple> for Target {
    fn from(value: Triple) -> Self {
        return Self { value };
    }
}

impl FromStr for Target {
    type Err = TargetParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let triple = Triple::from_str(s).map_err(TargetParseError::from)?;

        return Ok(Self {
            value: triple,
        });
    }
}

#[derive(Error, Debug)]
pub enum TargetParseError {
    UnrecognizedArchitecture(String),
    UnrecognizedVendor(String),
    UnrecognizedOperatingSystem(String),
    UnrecognizedEnvironment(String),
    UnrecognizedBinaryFormat(String),
    UnrecognizedField(String),
}

impl fmt::Display for TargetParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{:?}", self);
    }
}

impl From<TripleParseError> for TargetParseError {
    fn from(e: TripleParseError) -> Self {
        return match e {
            TripleParseError::UnrecognizedArchitecture(msg) => TargetParseError::UnrecognizedArchitecture(msg),
            TripleParseError::UnrecognizedVendor(msg) => TargetParseError::UnrecognizedVendor(msg),
            TripleParseError::UnrecognizedOperatingSystem(msg) => TargetParseError::UnrecognizedOperatingSystem(msg),
            TripleParseError::UnrecognizedEnvironment(msg) => TargetParseError::UnrecognizedEnvironment(msg),
            TripleParseError::UnrecognizedBinaryFormat(msg) => TargetParseError::UnrecognizedBinaryFormat(msg),
            TripleParseError::UnrecognizedField(msg) => TargetParseError::UnrecognizedField(msg),
        };
    }
}

