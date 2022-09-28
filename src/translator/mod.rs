pub mod ast;
mod error;
mod prep;

use ast::*;

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

fn translate_file(fe_file: parser::ast::FerrumFileAst) -> Result<RustFileAst> {
    let mut rs_file = RustFileAst { items: vec![] };

    for item in fe_file.items {
        let item = translate_item(item)?;

        rs_file.items.push(item);
    }

    return Ok(rs_file);
}

fn translate_item(item: parser::ast::ItemNode) -> Result<Item> {
    match item.item {
        parser::ast::Item::FnDef(fn_def) => {
            let fn_def = translate_fn_def(fn_def)?;
            return Ok(Item::FnDef(fn_def));
        }
        item => todo!("Cannot translate item: {item:#?}"),
    }
}

fn translate_fn_def(fn_def: parser::ast::FnDefNode) -> Result<FnDef> {
    return Ok(FnDef {
        name: fn_def.name.literal,
        params: vec![],
        return_type: None,
        body: fn_def
            .body
            .into_iter()
            .map(translate_stmt)
            .collect::<Result<Vec<Statement>>>()?,
    });
}

fn translate_stmt(stmt: parser::ast::StatementNode) -> Result<Statement> {
    match stmt.statement {
        parser::ast::Statement::Expr(expr) => {
            let expr = translate_expr(expr)?;
            return Ok(Statement::Expr(expr));
        }
        _ => todo!("Cannot translate statement: {stmt:#?}"),
    }
}

fn translate_expr(expr: parser::ast::ExprNode) -> Result<Expr> {
    match expr.expr {
        parser::ast::Expr::FnCall(fn_call) => {
            let fn_call = translate_fn_call(fn_call)?;
            return Ok(Expr::FnCall(fn_call));
        }
        _ => todo!("Cannot translate expression: {expr:#?}"),
    }
}

fn translate_fn_call(fn_call: parser::ast::FnCallNode) -> Result<FnCall> {
    return Ok(FnCall {
        name: fn_call.name.literal,
        args: vec![],
    });
}
