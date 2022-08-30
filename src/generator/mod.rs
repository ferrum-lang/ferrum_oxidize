mod error;

pub use error::GeneratorError;

use crate::translator::ast::RustProjectAst;
use crate::cargo::project::CargoProject;
use crate::Result;

use std::path::PathBuf;

pub fn generate_cargo_project(rust_project_ast: RustProjectAst, build_dir: PathBuf) -> Result<CargoProject> {
    todo!();
}
