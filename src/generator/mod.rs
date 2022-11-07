mod cargo;
use cargo::*;

mod codegen;
use codegen::*;

mod error;
pub use error::GeneratorError;

use crate::cargo::project::CargoProject;
use crate::translator::ast::*;
use crate::Result;

use std::{env, path::PathBuf};

const RUNTIME_RS: &'static str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/generated/runtime.rs"));

pub struct GenProject {
    pub main_file: GenFile,
    pub siblings: Vec<GenNode>,
}

#[derive(Debug, Clone)]
pub enum GenNode {
    File(GenFile),
    Dir(GenDir),
}

#[derive(Debug, Clone)]
pub struct GenFile {
    pub name: String,
    pub code: String,
}

#[derive(Debug, Clone)]
pub struct GenDir {
    pub name: String,
    pub files: Vec<GenNode>,
}

pub fn generate_cargo_project(
    rust_project: RustProject,
    build_dir: PathBuf,
) -> Result<CargoProject> {
    let mut gen_project = generate_rust_code(rust_project)?;

    gen_project.main_file.code.insert_str(0, "mod ferrum;\n");

    // println!(
    //     "\n*** Generated Rust Code ***\n{}\n*** End of Generated Code ***",
    //     gen_project.main_file.code.trim()
    // );

    let project = create_and_write_to_cargo_project(gen_project, build_dir)?;

    return Ok(project);
}

