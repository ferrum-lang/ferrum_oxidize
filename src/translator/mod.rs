mod error;
mod prep;

pub mod ast;
use ast::*;

pub mod translate;
use translate::*;

pub use error::TranslateError;

use crate::parser;
use crate::Result;

use std::collections::HashMap;

pub fn translate_to_rust(fe_ast: parser::ast::FerrumProjectAst) -> Result<RustProjectAst> {
    let fe_ast = prep::prepare_fe_ast_for_translation(fe_ast)?;

    let root = translate_to_rust_node(fe_ast.root)?;

    return Ok(RustProjectAst { root });
}

fn translate_to_rust_node(fe_node: parser::ast::FerrumProjectAstNode) -> Result<RustProjectAstNode> {
    let mut translator = Translator {
        is_mod_root: fe_node.file.is_mod_root,
        mods: fe_node.nodes
            .iter()
            .map(|node| node.file.name.clone())
            .collect(),
    };

    let mut file = translate_file(&mut translator, fe_node.file)?;

    for r#mod in translator.mods {
        file.items.insert(0, Item::Mod(Mod {
            is_public: false,
            name: r#mod,
        }));
    }

    let mut mods = HashMap::new();

    for fe_node in fe_node.nodes {
        let name = fe_node.file.name.clone();

        let node = translate_to_rust_node(fe_node)?;

        mods.insert(name, node);
    }

    return Ok(RustProjectAstNode {
        file,
        mods,
    });
}

