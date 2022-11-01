use super::*;

use crate::lexer::token::{Token, TokenKeyword, TokenType};
use crate::span::Span;

pub fn translate_fn_def(
    translator: &mut Translator,
    fn_def: parser::ast::FnDefNode,
) -> Result<FnDef> {
    let mut params = vec![];
    let mut default_value_stmts: Vec<parser::ast::ItemNode> = vec![];

    for mut param in fn_def.params.take_values() {
        if let Some(default_value) = &param.default_value {
            match param.param_type.typ {
                // Type::Option(_) => {}
                _ => {
                    param.param_type = parser::ast::TypeNode {
                        typ: parser::ast::Type::Optional(Box::new(param.param_type)),
                        span: Span::from((0, 0)),
                    };
                }
            }

            // default_value_stmts.push(parser::ast::ItemNode {
            //     item: parser::ast::Item::Statement(parser::ast::StatementNode {
            //         statement: parser::ast::Statement::Decl(parser::ast::DeclarationNode {
            //             decl_token: Token::new(
            //                 TokenType::Keyword(TokenKeyword::Const),
            //                 "const",
            //                 Span::from((0, 0)),
            //             ),
            //             is_const: true,
            //             assign_pattern: parser::ast::AssignPatternNode {
            //                 assign_pattern: parser::ast::AssignPattern::Id(param.name),
            //                 span: Span::from((0, 0)),
            //             },
            //             explicit_type: None,
            //             rhs_expr: Some((
            //                 Token::new(TokenType::Equals, "=", Span::from((0, 0))),
            //                 parser::ast::ExprNode {
            //                     expr: parser::ast::Expr::LhsOrElseRhs(...),
            //                     span: Span::from((0, 0)),
            //                 },
            //             )),
            //             span: Span::from((0, 0)),
            //         }),
            //         span: Span::from((0, 0)),
            //     }),
            //     span: Span::from((0, 0)),
            // });
        }

        params.push(translate_fn_def_param(translator, param)?);
    }

    translator.scope_stack.push(fn_def.scope);
    translator.scope_stack.push(HashMap::new());

    let mut body = vec![];

    for item in fn_def.body.get_items() {
        match item.item.clone() {
            parser::ast::Item::Statement(stmt) => body.push(translate_stmt(translator, stmt)?),
            _ => {
                for item in translate_item(translator, FeShared::get(&item).clone())? {
                    body.push(Statement::Item(item));
                }
            }
        }
    }

    translator.scope_stack.pop();
    translator.scope_stack.pop();

    return Ok(FnDef {
        is_public: fn_def.pub_token.is_some(),
        name: fn_def.name.literal,
        params,
        body,
        return_type: if let Some((_, return_type)) = fn_def.return_type {
            Some(translate_type(translator, return_type)?)
        } else {
            None
        },
    });
}

pub fn translate_fn_def_param(
    translator: &mut Translator,
    fn_def_param: parser::ast::FnDefParamNode,
) -> Result<FnDefParam> {
    return Ok(FnDefParam {
        name: fn_def_param.name.literal,
        param_type: translate_type(translator, fn_def_param.param_type)?,
    });
}
