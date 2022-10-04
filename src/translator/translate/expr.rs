use super::*;

pub fn translate_expr(translator: &mut Translator, expr: parser::ast::ExprNode) -> Result<Expr> {
    match expr.expr {
        parser::ast::Expr::FnCall(fn_call) => {
            let fn_call = translate_fn_call(translator, fn_call)?;
            return Ok(Expr::FnCall(fn_call));
        }
        parser::ast::Expr::Literal(literal) => {
            let literal = translate_literal(translator, literal)?;
            return Ok(Expr::Literal(literal));
        }
        parser::ast::Expr::IdentLookup(ident_lookup) => {
            return Ok(Expr::IdentLookup(ident_lookup.name.literal));
        },
        parser::ast::Expr::Ref(ref_node) => {
            let expr = translate_expr(translator, *ref_node.expr)?;
            
            if ref_node.mut_token.is_some() {
                return Ok(Expr::MutRef(Box::new(expr)));
            } else {
                return Ok(Expr::SharedRef(Box::new(expr)));
            }
        },
    }
}

pub fn translate_fn_call(translator: &mut Translator, fn_call: parser::ast::FnCallNode) -> Result<FnCall> {
    return Ok(FnCall {
        name: fn_call.name.literal,
        args: fn_call
            .args
            .take_values()
            .into_iter()
            .map(|call_arg| translate_expr(translator, *call_arg.expr))
            .collect::<Result<Vec<Expr>>>()?,
    });
}

pub fn translate_literal(translator: &mut Translator, literal: parser::ast::LiteralNode) -> Result<Literal> {
    match literal.literal {
        parser::ast::Literal::Bool(is_true) => {
            return Ok(Literal::Bool(is_true));
        },
        parser::ast::Literal::String(string) => {
            return Ok(Literal::String(string));
        },
    }
}

