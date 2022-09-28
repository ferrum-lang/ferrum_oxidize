mod error;

pub use error::GeneratorError;

use crate::translator::ast::*;
use crate::cargo::project::CargoProject;
use crate::Result;

use std::path::PathBuf;

pub fn generate_cargo_project(rust_project_ast: RustProjectAst, build_dir: PathBuf) -> Result<CargoProject> {
    let mut rs = String::new();

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
                                },
                            }

                            rs.push_str(";\n");
                        },
                        _ => todo!("{stmt:#?}"),
                    }
                }

                rs.push_str("}\n");
            },
            _ => todo!("{item:#?}"),
        }
    }

    todo!("\n\n*** Generated Rust Code ***\n{}\n*** End of Generated Code ***\n\n", rs.trim());
}
