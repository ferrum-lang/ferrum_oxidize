use super::*;

pub fn translate_stmt(stmt: parser::ast::StatementNode) -> Result<Statement> {
    match stmt.statement {
        parser::ast::Statement::Expr(expr) => {
            let expr = translate_expr(expr)?;
            return Ok(Statement::Expr(expr));
        },
        parser::ast::Statement::Decl(decl) => {
            let decl = translate_decl(decl)?;
            return Ok(Statement::Decl(decl));
        },
        _ => todo!("Cannot translate statement: {stmt:#?}"),
    }
}

pub fn translate_decl(decl: parser::ast::DeclarationNode) -> Result<Declaration> {
    return Ok(Declaration {
        is_const: decl.is_const,
        assign_pattern: translate_assign_pattern(decl.assign_pattern)?,
        explicit_type: if let Some(explicit_type) = decl.explicit_type {
            Some(translate_type(explicit_type.1)?)
        } else {
            None
        },
        rhs: if let Some(rhs_expr) = decl.rhs_expr {
            Some(translate_expr(rhs_expr.1)?)
        } else {
            None
        },
    });
}

pub fn translate_assign_pattern(assign_pattern: parser::ast::AssignPatternNode) -> Result<AssignPattern> {
    match assign_pattern.assign_pattern {
        parser::ast::AssignPattern::Id(id) => return Ok(AssignPattern::Id(id.literal)),
    }
}

