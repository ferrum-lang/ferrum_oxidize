use super::*;

pub fn translate_expr(expr: parser::ast::ExprNode) -> Result<Expr> {
    match expr.expr {
        parser::ast::Expr::FnCall(fn_call) => {
            let fn_call = translate_fn_call(fn_call)?;
            return Ok(Expr::FnCall(fn_call));
        }
        parser::ast::Expr::Literal(literal) => {
            let literal = translate_literal(literal)?;
            return Ok(Expr::Literal(literal));
        }
        // parser::ast::Expr::IdentLookup(ident_lookup) => {
        //     let ident_lookup = translate_ident_lookup(literal)?;
        //     return Ok(Expr::IdentLookup(ident_lookup));
        // },
        _ => todo!("Cannot translate expression: {expr:#?}"),
    }
}

pub fn translate_fn_call(fn_call: parser::ast::FnCallNode) -> Result<FnCall> {
    return Ok(FnCall {
        name: fn_call.name.literal,
        args: fn_call
            .args
            .take_values()
            .into_iter()
            .map(|call_arg| translate_expr(*call_arg.expr))
            .collect::<Result<Vec<Expr>>>()?,
    });
}

pub fn translate_literal(literal: parser::ast::LiteralNode) -> Result<Literal> {
    match literal.literal {
        parser::ast::Literal::Bool(is_true) => {
            return Ok(Literal::Bool(is_true));
        },
        parser::ast::Literal::String(string) => {
            return Ok(Literal::String(string));
        },
    }
}

