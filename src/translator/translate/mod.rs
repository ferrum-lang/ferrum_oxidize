use super::*;

mod expr;
use expr::*;

mod fn_def;
use fn_def::*;

mod stmt;
use stmt::*;

mod r#type;
use r#type::*;

use crate::parser;
use crate::Result;

pub fn translate_file(fe_file: parser::ast::FerrumFileAst) -> Result<RustFileAst> {
    let mut rs_file = RustFileAst { items: vec![] };

    for item in fe_file.items {
        let item = translate_item(item)?;

        rs_file.items.push(item);
    }

    return Ok(rs_file);
}

pub fn translate_item(item: parser::ast::ItemNode) -> Result<Item> {
    match item.item {
        parser::ast::Item::FnDef(fn_def) => {
            let fn_def = translate_fn_def(fn_def)?;
            return Ok(Item::FnDef(fn_def));
        }
        item => todo!("Cannot translate item: {item:#?}"),
    }
}

