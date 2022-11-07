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
        parser::ast::Expr::StaticAccess(static_access) => {
            let static_access = translate_static_access(translator, static_access)?;
            return Ok(Expr::StaticAccess(static_access));
        }
        parser::ast::Expr::IdentLookup(ident_lookup) => {
            return Ok(Expr::IdentLookup(ident_lookup.name.literal));
        }
        parser::ast::Expr::Ref(ref_node) => {
            let expr = translate_expr(translator, *ref_node.expr)?;

            if ref_node.mut_token.is_some() {
                return Ok(Expr::MutRef(Box::new(expr)));
            } else {
                return Ok(Expr::SharedRef(Box::new(expr)));
            }
        }
        parser::ast::Expr::Deref(deref_node) => {
            let expr = translate_expr(translator, *deref_node.expr)?;
            return Ok(Expr::Deref(Box::new(expr)));
        }
    }
}

pub fn translate_fn_call(
    translator: &mut Translator,
    fn_call: parser::ast::FnCallNode,
) -> Result<FnCall> {
    if let None = translator.find_in_scope(&fn_call.name.literal) {
        use crate::parser::ast::ScopeTable;
        let mut god_scope = ScopeTable::new();

        for scope in &translator.scope_stack {
            god_scope.extend(scope.clone());
        }

        dbg!(god_scope);
        todo!("{fn_call:#?}");
    }

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
        }
        parser::ast::Literal::String(string) => {
            return Ok(Literal::String(string));
        }
        parser::ast::Literal::SomeOption { expr, .. } => {
            let expr = Box::new(translate_expr(translator, *expr)?);
            return Ok(Literal::SomeOption(expr));
        }
        parser::ast::Literal::NoneOption(_) => {
            return Ok(Literal::NoneOption);
        }
    }
}

pub fn translate_static_access(
    translator: &mut Translator,
    static_access: parser::ast::StaticAccessNode,
) -> Result<StaticAccess> {
    use parser::ast::scope::*;

    let lhs = static_access.lhs.name.literal;

    let inner_scope = match translator.find_in_scope(&lhs) {
        Some(ScopeRefNode {
            scope_ref: ScopeRef::Mod(scope),
            ..
        }) => scope,
        found => {
            dbg!(&lhs);
            dbg!(found);
            todo!();
        }
    };

    let mut inner_scope = vec![inner_scope];
    std::mem::swap(&mut translator.scope_stack, &mut inner_scope);
    let mut outer_scope = inner_scope;

    let rhs = translate_expr(translator, *static_access.rhs)?;

    std::mem::swap(&mut translator.scope_stack, &mut outer_scope);

    return Ok(StaticAccess {
        lhs,
        rhs: Box::new(rhs),
    });
}
