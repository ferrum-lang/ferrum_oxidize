use super::*;

pub fn translate_stmt(stmt: parser::ast::StatementNode) -> Result<Statement> {
    match stmt.statement {
        parser::ast::Statement::Expr(expr) => {
            let expr = translate_expr(expr)?;
            return Ok(Statement::Expr(expr));
        },
        parser::ast::Statement::Assign(assign) => {
            let assign = translate_assign(assign)?;
            return Ok(Statement::Assign(assign));
        },
        _ => todo!("Cannot translate statement: {stmt:#?}"),
    }
}

pub fn translate_assign(assign: parser::ast::AssignNode) -> Result<Assign> {
    return Ok(Assign {
        is_const: assign.is_const,
        name: assign.name.literal,
        explicit_type: if let Some(explicit_type) = assign.explicit_type {
            Some(translate_type(explicit_type.1)?)
        } else {
            None
        },
        rhs: if let Some(rhs_expr) = assign.rhs_expr {
            Some(translate_expr(rhs_expr.1)?)
        } else {
            None
        },
    });
}

