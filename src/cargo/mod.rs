mod error;
pub mod project;

pub use error::CargoError;

use crate::Target;
use crate::Result;

use std::path::PathBuf;

pub fn build(cargo_project: project::CargoProject, target: Target, out_file: PathBuf) -> Result {
    todo!();
}

