use super::*;

pub fn parse_fn_def(parser: &mut Parser, pub_token: Option<Token>) -> Result<FnDefNode> {
    let fn_token = parser.consume(TokenType::Keyword(TokenKeyword::Fn))?;

    let name = parser.consume(TokenType::Identifier)?;

    let open_paren = parser.consume(TokenType::OpenParenthesis)?;

    let mut params = Punctuated::new();
    let mut prev_comma = None;

    while !parser.scan(&[TokenType::CloseParenthesis]) {
        let name = parser.consume(TokenType::Identifier)?;

        let colon = parser.consume(TokenType::Colon)?;

        let param_type = parse_type(parser)?;

        let default_value = if parser.scan(&[TokenType::Equals]) {
            let eq = parser.consume(TokenType::Equals)?;
            let expr = parse_expr(parser)?;

            Some((eq, expr))
        } else {
            None
        };

        let span = if let Some((_, expr)) = &default_value {
            Span::from((name.span, expr.span))
        } else {
            Span::from((name.span, param_type.span))
        };

        params.push(
            prev_comma,
            FnDefParamNode {
                span,
                name,
                colon,
                param_type,
                default_value,
            },
        );

        prev_comma = parser.consume_if(TokenType::Comma)?;
    }

    if let Some(prev_comma) = prev_comma {
        params.push_trailing(prev_comma);
    }

    let close_paren = parser.consume(TokenType::CloseParenthesis)?;

    let return_type = if parser.scan(&[TokenType::SkinnyArrow]) {
        let arrow = parser.consume(TokenType::SkinnyArrow)?;
        let return_type = parse_type(parser)?;

        Some((arrow, return_type))
    } else {
        None
    };

    let (body, span) = if let Some(fat_arrow) = parser.consume_if(TokenType::FatArrow)? {
        let stmt = parse_statement(parser)?;

        let body_span = Span::from((fat_arrow.span, stmt.span));

        let span = if let Some(pub_token) = &pub_token {
            Span::from((pub_token.span, body_span))
        } else {
            Span::from((fn_token.span, body_span))
        };

        let body = FnDefBody::Stmt(FnDefStmtNode {
            fat_arrow,
            stmt: Box::new(FeShared::new(ItemNode {
                span: stmt.span,
                item: Item::Statement(stmt),
            })),
            span,
        });

        (body, span)
    } else {
        let items = parse_items_while(parser, |parser| !parser.scan(&[TokenType::Semicolon]))?;

        for item in &items {
            match &item.item {
                Item::Statement(_) => {}
                Item::Use(node) => {
                    if node.public.is_some() {
                        todo!("Error: Functions cannot export internal items");
                    }
                }
                Item::FnDef(node) => {
                    if node.pub_token.is_some() {
                        todo!("Error: Functions cannot export internal items");
                    }
                }
            }
        }

        let close_semicolon = parser.consume(TokenType::Semicolon)?;

        let block_span = if let Some(stmt) = items.first() {
            Span::from((stmt.span, close_semicolon.span))
        } else {
            close_semicolon.span
        };

        let span = if let Some(pub_token) = &pub_token {
            Span::from((pub_token.span, block_span))
        } else {
            Span::from((fn_token.span, block_span))
        };

        let body = FnDefBody::Block(FnDefBlockNode {
            items,
            close_semicolon,
            span: block_span,
        });

        (body, span)
    };

    return Ok(FnDefNode {
        pub_token,
        fn_token,
        name,
        generics: None,
        open_paren,
        params,
        close_paren,
        return_type,
        body,
        span,
        scope: ScopeTable::new(),
    });
}
