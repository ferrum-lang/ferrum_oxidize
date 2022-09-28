mod error;

pub use error::GeneratorError;

use crate::cargo::project::CargoProject;
use crate::translator::ast::*;
use crate::Result;

use std::path::PathBuf;

pub fn generate_cargo_project(
    rust_project_ast: RustProjectAst,
    build_dir: PathBuf,
) -> Result<CargoProject> {
    let mut rs = String::from("pub fn print() { println!(\"Hello world!\"); }\n\n");

    for item in rust_project_ast.root.file.items {
        match item {
            Item::FnDef(fn_def) => {
                rs.push_str(&format!("fn {}() {{\n", fn_def.name));

                for stmt in fn_def.body {
                    match stmt {
                        Statement::Expr(expr) => {
                            match expr {
                                Expr::FnCall(fn_call) => {
                                    rs.push_str(&format!("  {}()", fn_call.name));
                                }
                            }

                            rs.push_str(";\n");
                        }
                        _ => todo!("{stmt:#?}"),
                    }
                }

                rs.push_str("}\n");
            }
            _ => todo!("{item:#?}"),
        }
    }

    println!(
        "\n\n*** Generated Rust Code ***\n{}\n*** End of Generated Code ***\n\n",
        rs.trim()
    );

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

    dbg!(&src_file);

    std::fs::write(src_file, rs)?;

    return Ok(CargoProject {});
}

