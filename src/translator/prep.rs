use super::*;

use crate::lexer;
use crate::punctuated::Punctuated;
use crate::span::Span;

pub fn prepare_fe_ast_for_translation(
    mut fe_ast: parser::ast::FerrumProjectAst,
) -> Result<parser::ast::FerrumProjectAst> {
    move_top_stmts_to_main(&mut fe_ast);

    ensure_no_top_stmts(&fe_ast)?;

    return Ok(fe_ast);
}

fn move_top_stmts_to_main(fe_ast: &mut parser::ast::FerrumProjectAst) {
    if has_main_fn(&fe_ast) {
        return;
    }

    let mut items = vec![];

    std::mem::swap(&mut fe_ast.root.file.items, &mut items);

    let mut iter = items.into_iter();

    let mut stmts = vec![];
    while let Some(node) = iter.next() {
        match node.item {
            parser::ast::Item::Statement(_) => stmts.push(node),
            _ => fe_ast.root.file.items.push(node),
        }
    }

    let span = Span::from((fe_ast.root.file.span.to.line + 1, 1));

    fe_ast.root.file.items.push(parser::ast::ItemNode {
        item: parser::ast::Item::FnDef(parser::ast::FnDefNode {
            pub_token: None,
            fn_token: lexer::token::Token {
                literal: String::from("fn"),
                token_type: lexer::token::TokenType::Keyword(
                    lexer::token::TokenKeyword::Fn
                ),
                span,
            },
            name: lexer::token::Token {
                literal: String::from("main"),
                token_type: lexer::token::TokenType::Identifier,
                span,
            },
            generics: None,
            open_paren: lexer::token::Token {
                literal: String::from("("),
                token_type: lexer::token::TokenType::OpenParenthesis,
                span,
            },
            params: Punctuated::new(),
            close_paren: lexer::token::Token {
                literal: String::from(")"),
                token_type: lexer::token::TokenType::CloseParenthesis,
                span,
            },
            return_type: None,
            open_brace: lexer::token::Token {
                literal: String::from("{"),
                token_type: lexer::token::TokenType::OpenBrace,
                span,
            },
            body: stmts,
            close_brace: lexer::token::Token {
                literal: String::from("}"),
                token_type: lexer::token::TokenType::CloseBrace,
                span,
            },
            scope: fe_ast.root.file.scope.clone(),
            span,
        }),
        span,
    });
}

fn has_main_fn(fe_ast: &parser::ast::FerrumProjectAst) -> bool {
    for node in &fe_ast.root.file.items {
        if let parser::ast::Item::FnDef(fn_def) = &node.item {
            if fn_def.name.token_type == lexer::token::TokenType::Identifier
                && fn_def.name.literal.as_str() == "main"
            {
                return true;
            }
        }
    }

    return false;
}

fn ensure_no_top_stmts(fe_ast: &parser::ast::FerrumProjectAst) -> Result {
    fn ensure_no_top_stmts_in_node(fe_node: &parser::ast::FerrumProjectAstNode) -> Result {
        for node in &fe_node.file.items {
            if let parser::ast::Item::Statement(stmt) = &node.item {
                Err(TranslateError::InvalidTopLevelStatement(
                    file!(),
                    line!(),
                    stmt.clone(),
                ))?;
            }
        }

        return Ok(());
    }

    return ensure_no_top_stmts_in_node(&fe_ast.root);
}

