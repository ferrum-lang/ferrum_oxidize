mod cargo;
use cargo::*;

mod codegen;
use codegen::*;

mod error;
pub use error::GeneratorError;

use crate::cargo::project::CargoProject;
use crate::translator::ast::*;
use crate::Result;

use std::{env, path::PathBuf, collections::HashMap};

const RUNTIME_RS: &'static str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/generated/runtime.rs"));

#[derive(Debug, Clone)]
pub struct GenFile {
    code: String,
    mods: HashMap<String, GenFile>,
}

pub fn generate_cargo_project(
    rust_project_ast: RustProjectAst,
    build_dir: PathBuf,
) -> Result<CargoProject> {
    let mut root_file = generate_rust_code(rust_project_ast.root)?;

    root_file.code.insert_str(0, "mod ferrum;\n");

    println!(
        "\n*** Generated Rust Code ***\n{}\n*** End of Generated Code ***",
        root_file.code.trim()
    );

    let project = create_and_write_to_cargo_project(root_file, build_dir)?;

    return Ok(project);
}

