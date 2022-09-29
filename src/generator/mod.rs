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

pub fn generate_cargo_project(
    rust_project_ast: RustProjectAst,
    build_dir: PathBuf,
) -> Result<CargoProject> {
    let mut root_file = generate_rust_code(rust_project_ast.root)?;

    root_file.code.insert_str(0, "mod ferrum;\n");

    println!(
        "\n\n*** Generated Rust Code ***\n{}\n*** End of Generated Code ***\n\n",
        root_file.code.trim()
    );

    return create_and_write_to_cargo_project(root_file, build_dir);
}

pub struct Generator {
    pub indent_count: usize,
}

impl Generator {
    pub fn new() -> Self {
        return Self {
            indent_count: 0,
        };
    }

    pub fn padding(&self) -> String {
        let mut left_pad = String::new();

        for _ in 0..self.indent_count {
            left_pad.push_str("    ");
        }

        return left_pad;
    }
}

pub struct GenFile {
    code: String,
    nodes: Vec<GenFile>,
}

pub fn generate_rust_code(ast_node: RustProjectAstNode) -> Result<GenFile> {
    let mut generator = Generator::new();
    let rs = gen_rs_for_file(&mut generator, ast_node.file)?;

    let mut file = GenFile {
        code: rs,
        nodes: vec![],
    };

    for child in ast_node.nodes {
        let child_file = generate_rust_code(child)?;
        file.nodes.push(child_file);
    }

    return Ok(file);
}

pub fn create_and_write_to_cargo_project(root_file: GenFile, build_dir: PathBuf) -> Result<CargoProject> {
    if let Err(_) = std::fs::canonicalize(&build_dir) {
        let output = std::process::Command::new("cargo")
            .args(&[
                "new",
                "--name",
                "main",
                "--vcs",
                "none",
                "--color",
                "never",
                "--quiet",
                &build_dir.to_string_lossy(),
            ])
            .output()?;

        if !output.status.success() {
            let stderr = output.stderr;
            let string = String::from_utf8(stderr)?;

            eprintln!("{}", string);

            todo!();
        }

        let stdout = output.stdout;
        let string = String::from_utf8(stdout)?;

        todo!("{}", string);
    } else {
        println!("No need to build");
    }

    let src_file = build_dir.join("src/main.rs");
    std::fs::write(src_file, root_file.code)?;

    let lib_file = build_dir.join("src/ferrum.rs");
    std::fs::write(lib_file, RUNTIME_RS)?;

    return Ok(CargoProject {});
}
