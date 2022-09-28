use super::*;

pub fn parse_file(parser: &mut Parser) -> Result<FerrumFileAst> {
    let mut ast = FerrumFileAst::new();

    while parser.index < parser.tokens.len() {
        let item = parse_item(parser)?;

        if let Some(next) = parser.current().ok() {
            if next.span.from.line == item.span.to.line {
                Err(ParseError::NotExpectedNewline(file!(), line!(), next))?;
            }
        }

        ast.items.push(item);
    }

    match parser.tokens.len() {
        0 => {}
        1 => ast.span = parser.tokens[0].span,
        n => ast.span = Span::from((parser.tokens[0].span, parser.tokens[n - 1].span)),
    }

    return Ok(ast);
}

fn parse_item(parser: &mut Parser) -> Result<ItemNode> {
    let token = parser
        .current()
        .with_context(|| format!("Expected some item to parse"))?;

    match token.token_type {
        _ => {
            let statement = parse_statement(parser)?;

            return Ok(ItemNode {
                span: statement.span,
                item: Item::Statement(statement),
            });
        }
    }
}

fn parse_statement(parser: &mut Parser) -> Result<StatementNode> {
    let token = parser
        .current()
        .with_context(|| format!("Expected some statement to parse"))?;

    match token.token_type {
        TokenType::Identifier => return parse_ident_stmt(parser),
        _ => {
            let expr = parse_expr(parser)?;

            return Ok(StatementNode {
                span: expr.span,
                statement: Statement::Expr(expr),
            });
        }
    }
}

fn parse_ident_stmt(parser: &mut Parser) -> Result<StatementNode> {
    parser.expect(TokenType::Identifier)?;

    let token = parser.next().ok();

    match token.map(|t| t.token_type) {
        _ => {
            let expr = parse_ident_expr(parser)?;

            return Ok(StatementNode {
                span: expr.span,
                statement: Statement::Expr(expr),
            });
        }
    }
}

fn parse_expr(parser: &mut Parser) -> Result<ExprNode> {
    let token = parser
        .current()
        .with_context(|| format!("Expected some expr to parse"))?;

    match token.token_type {
        TokenType::Identifier => {
            let ident_expr = parse_ident_expr(parser)?;
            return Ok(ident_expr);
        }
        _ => Err(ParseError::UnexpectedToken(file!(), line!(), token))?,
    }
}

fn parse_ident_expr(parser: &mut Parser) -> Result<ExprNode> {
    parser.expect(TokenType::Identifier)?;

    let token = parser.next().ok();

    match token.map(|t| t.token_type) {
        Some(TokenType::OpenParenthesis) => {
            let fn_call = parse_fn_call(parser)?;

            return Ok(ExprNode {
                span: fn_call.span,
                expr: Expr::FnCall(fn_call),
            });
        }
        _ => {
            let ident_lookup = parse_ident_lookup(parser)?;

            return Ok(ExprNode {
                span: ident_lookup.span,
                expr: Expr::IdentLookup(ident_lookup),
            });
        }
    }
}

fn parse_ident_lookup(parser: &mut Parser) -> Result<IdentLookupNode> {
    let ident_token = parser.consume(TokenType::Identifier)?;

    return Ok(IdentLookupNode {
        span: ident_token.span,
        name: ident_token,
    });
}

fn parse_fn_call(parser: &mut Parser) -> Result<FnCallNode> {
    let ident_token = parser.consume(TokenType::Identifier)?;

    let open_token = parser.consume(TokenType::OpenParenthesis)?;

    let mut args = Punctuated::new();
    let mut prev_comma = None;

    while !parser.scan(&[TokenType::CloseParenthesis]) {
        let name = if parser.scan(&[TokenType::Identifier, TokenType::Equals]) {
            let name = parser.consume(TokenType::Identifier)?;
            let eq = parser.consume(TokenType::Equals)?;

            Some((name, eq))
        } else {
            None
        };

        let expr = Box::new(parse_expr(parser)?);

        let span = if let Some((name, _)) = &name {
            Span::from((name.span, expr.span))
        } else {
            expr.span
        };

        args.push(prev_comma, FnCallArgNode { span, name, expr });

        prev_comma = parser.consume_if(TokenType::Comma);
    }

    if let Some(prev_comma) = prev_comma {
        args.push_trailing(prev_comma);
    }

    let close_token = parser.consume(TokenType::CloseParenthesis)?;

    return Ok(FnCallNode {
        span: Span::from((ident_token.span, close_token.span)),
        name: ident_token,
        open_paren: open_token,
        close_paren: close_token,
        args,
    });
}
