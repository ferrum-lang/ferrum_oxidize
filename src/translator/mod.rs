pub mod ast;
mod error;

use ast::RustProjectAst;

pub use error::TranslatorError;

use crate::parser;
use crate::Result;

pub fn translate_to_rust(ferrum_project_ast: parser::ast::FerrumProjectAst) -> Result<RustProjectAst> {
    todo!();
}

