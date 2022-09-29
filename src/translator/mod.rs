mod error;
mod prep;

pub mod ast;
use ast::*;

pub mod translate;
use translate::*;

pub use error::TranslateError;

use crate::parser;
use crate::Result;

pub fn translate_to_rust(fe_ast: parser::ast::FerrumProjectAst) -> Result<RustProjectAst> {
    let fe_ast = prep::prepare_fe_ast_for_translation(fe_ast)?;

    let rs_root_file = translate_file(fe_ast.root.file)?;

    let mut rs_ast = RustProjectAst {
        root: RustProjectAstNode {
            file: rs_root_file,
            nodes: vec![],
        },
    };

    return Ok(rs_ast);
}

