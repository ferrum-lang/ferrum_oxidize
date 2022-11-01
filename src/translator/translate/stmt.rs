use super::*;

pub fn translate_stmt(
    translator: &mut Translator,
    stmt: parser::ast::StatementNode,
) -> Result<Statement> {
    use crate::parser::ast::scope::*;

    match stmt.statement {
        parser::ast::Statement::Pass => {
            return Ok(Statement::Expr(Expr::Literal(Literal::Tuple(vec![]))));
        },
        parser::ast::Statement::Do(do_node) => {
            let block = translate_do(translator, do_node)?;
            return Ok(Statement::Block(block));
        },
        parser::ast::Statement::Expr(expr) => {
            let expr = translate_expr(translator, expr)?;
            return Ok(Statement::Expr(expr));
        }
        parser::ast::Statement::Decl(decl) => {
            let explicit_type = decl.explicit_type.clone().map(|(_, b)| b.typ);

            let decl = translate_decl(translator, decl)?;

            match &decl.decl_pattern {
                DeclPattern::Id(name) => {
                    translator.scope_stack.last_mut().unwrap().insert(
                        name.clone(),
                        ScopeRefNode {
                            name: name.clone(),
                            is_public: false,
                            scope_ref: ScopeRef::LocalVar {
                                is_const: decl.is_const,
                                name: name.clone(),
                                known_type: explicit_type,
                            },
                        },
                    );
                }
            }

            return Ok(Statement::Decl(decl));
        }
        parser::ast::Statement::Assign(assign) => {
            let assign = translate_assign(translator, assign)?;
            return Ok(Statement::Assign(assign));
        }
    }
}

pub fn translate_decl(
    translator: &mut Translator,
    decl: parser::ast::DeclarationNode,
) -> Result<Declaration> {
    return Ok(Declaration {
        is_const: decl.is_const,
        decl_pattern: translate_decl_pattern(translator, decl.decl_pattern)?,
        explicit_type: if let Some(explicit_type) = decl.explicit_type {
            Some(translate_type(translator, explicit_type.1)?)
        } else {
            None
        },
    });
}

pub fn translate_decl_pattern(
    _: &mut Translator,
    decl_pattern: parser::ast::DeclPatternNode,
) -> Result<DeclPattern> {
    match decl_pattern.decl_pattern {
        parser::ast::DeclPattern::Id(id) => return Ok(DeclPattern::Id(id.literal)),
    }
}

pub fn translate_assign(
    translator: &mut Translator,
    assign: parser::ast::AssignNode,
) -> Result<Assign> {
    let lhs = translate_stmt(translator, *assign.lhs)?;
    let rhs = translate_expr(translator, assign.rhs)?;

    let assign_type = translate_assign_type(translator, assign.assign)?;

    return Ok(Assign {
        lhs: Box::new(lhs),
        rhs,
        assign_type,
    });
}

pub fn translate_assign_type(_: &mut Translator, assign_type: parser::ast::Token) -> Result<AssignType> {
    use crate::lexer::token::*;

    match assign_type.token_type {
        TokenType::Equals => return Ok(AssignType::Eq),
        _ => todo!(),
    }
}

fn translate_do(translator: &mut Translator, do_node: parser::ast::DoNode) -> Result<StmtBlock> {
    let mut block = StmtBlock {
        stmts: vec![],
    };

    for stmt in do_node.stmts {
        block.stmts.push(translate_stmt(translator, stmt)?);
    }

    return Ok(block);
}
