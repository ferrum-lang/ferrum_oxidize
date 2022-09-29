use super::*;

pub fn parse_expr(parser: &mut Parser) -> Result<ExprNode> {
    let token = parser
        .current()
        .with_context(|| format!("Expected some expr to parse"))?;

    match token.token_type {
        TokenType::Identifier => return parse_ident_expr(parser),
        TokenType::Literal(_) => {
            let literal = parse_literal(parser)?;

            return Ok(ExprNode {
                span: literal.span,
                expr: Expr::Literal(literal),
            });
        },
        _ => Err(ParseError::UnexpectedToken(file!(), line!(), token))?,
    }
}

pub fn parse_literal(parser: &mut Parser) -> Result<LiteralNode> {
    let token = parser
        .current()
        .with_context(|| format!("Expected some expr to parse"))?;

    let literal = if let TokenType::Literal(literal) = token.token_type {
        literal
    } else {
        Err(ParseError::UnexpectedToken(file!(), line!(), token.clone()))?
    };

    parser.index += 1;

    match literal {
        TokenLiteral::Bool(is_true) => {
            return Ok(LiteralNode {
                span: token.span,
                literal: Literal::Bool(is_true),
            });
        },
        TokenLiteral::String => {
            return Ok(LiteralNode {
                span: token.span,
                literal: Literal::String(token.literal),
            });
        },
        _ => todo!(),
    }
}

pub fn parse_ident_expr(parser: &mut Parser) -> Result<ExprNode> {
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

pub fn parse_ident_lookup(parser: &mut Parser) -> Result<IdentLookupNode> {
    let ident_token = parser.consume(TokenType::Identifier)?;

    return Ok(IdentLookupNode {
        span: ident_token.span,
        name: ident_token,
    });
}

pub fn parse_fn_call(parser: &mut Parser) -> Result<FnCallNode> {
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
