use super::*;

pub fn translate_stmt(stmt: parser::ast::StatementNode) -> Result<Statement> {
    match stmt.statement {
        parser::ast::Statement::Expr(expr) => {
            let expr = translate_expr(expr)?;
            return Ok(Statement::Expr(expr));
        }
        _ => todo!("Cannot translate statement: {stmt:#?}"),
    }
}

