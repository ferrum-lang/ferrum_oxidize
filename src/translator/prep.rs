use super::*;

use crate::lexer;
use crate::punctuated::Punctuated;
use crate::span::Span;

use std::collections::HashSet;

pub fn prepare_fe_ast_for_translation(
    root_node: &mut FeShared<parser::ast::FerrumModNode>,
) -> Result {
    match &mut root_node.file {
        parser::ast::FerrumModNodeFile::File(file) => move_top_stmts_to_main(file),
        parser::ast::FerrumModNodeFile::Dir(_) => todo!(),
    }
    ensure_no_top_stmts(&root_node)?;

    return Ok(());
}

fn move_top_stmts_to_main(file_ast: &mut parser::ast::FerrumFileAst) {
    if has_main_fn(&file_ast) {
        return;
    }

    let mut items = vec![];

    std::mem::swap(&mut file_ast.items, &mut items);

    let mut iter = items.into_iter();

    let mut stmts = vec![];
    while let Some(node) = iter.next() {
        match node.item {
            parser::ast::Item::Statement(_) => stmts.push(node),
            _ => file_ast.items.push(node),
        }
    }

    let span = Span::from((file_ast.span.to.line + 1, 1));

    let item = FeShared::new(parser::ast::ItemNode {
        item: parser::ast::Item::FnDef(parser::ast::FnDefNode {
            pub_token: None,
            fn_token: lexer::token::Token {
                literal: String::from("fn"),
                token_type: lexer::token::TokenType::Keyword(lexer::token::TokenKeyword::Fn),
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
            scope: file_ast.scope.clone(),
            span,
        }),
        span,
    });

    file_ast.items.push(item);
}

fn has_main_fn(file_ast: &parser::ast::FerrumFileAst) -> bool {
    for node in &file_ast.items {
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

fn ensure_no_top_stmts(root_node: &FeShared<parser::ast::FerrumModNode>) -> Result {
    fn rec_ensure_no_top_stmts(
        node: &FeShared<parser::ast::FerrumModNode>,
        visited: &mut HashSet<usize>,
    ) -> Result {
        if visited.contains(&node.id) {
            return Ok(());
        }

        match &node.file {
            parser::ast::FerrumModNodeFile::File(file) => {
                for item in &file.items {
                    if let parser::ast::Item::Statement(stmt) = &item.item {
                        Err(TranslateError::InvalidTopLevelStatement(
                            file!(),
                            line!(),
                            stmt.clone(),
                        ))?;
                    }
                }
            }
            parser::ast::FerrumModNodeFile::Dir(nodes) => {
                for (_, node) in nodes {
                    rec_ensure_no_top_stmts(node, visited)?;
                }
            }
        }

        visited.insert(node.id);

        for (_, sibling_ref) in &node.sibling_refs {
            rec_ensure_no_top_stmts(sibling_ref, visited)?;
        }

        return Ok(());
    }

    return rec_ensure_no_top_stmts(root_node, &mut HashSet::new());
}
