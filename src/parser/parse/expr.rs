use super::*;

pub fn parse_expr(parser: &mut Parser) -> Result<ExprNode> {
    let expr = parse_expr_layer_2(parser)?;

    let token = parser.next().ok();

    match token.map(|t| t.token_type) {
        Some(TokenType::Plus) => {
            todo!();
        }
        _ => return Ok(expr),
    }
}

pub fn parse_expr_layer_2(parser: &mut Parser) -> Result<ExprNode> {
    let token = parser
        .current()
        .with_context(|| format!("Expected some expr to parse"))?;

    let expr = match token.token_type {
        TokenType::Ampersand => parse_ref_expr(parser)?,
        TokenType::At => parse_deref_expr(parser)?,
        _ => parse_expr_layer_3(parser)?,
    };

    return Ok(expr);
}

pub fn parse_expr_layer_3(parser: &mut Parser) -> Result<ExprNode> {
    let token = parser
        .current()
        .with_context(|| format!("Expected some expr to parse"))?;

    let expr = match token.token_type {
        TokenType::Identifier => parse_ident_expr(parser)?,
        TokenType::Literal(_) => {
            let literal = parse_literal(parser)?;

            ExprNode {
                span: literal.span,
                expr: Expr::Literal(literal),
            }
        }
        _ => Err(ParseError::UnexpectedToken(file!(), line!(), token))?,
    };

    let token = parser.current().ok();

    match token.map(|t| t.token_type) {
        _ => return Ok(expr),
    }
}

pub fn parse_deref_expr(parser: &mut Parser) -> Result<ExprNode> {
    let deref_token = parser.consume(TokenType::At)?;

    let expr = parse_expr_layer_2(parser)?;

    let span = Span::from((deref_token.span, expr.span));

    return Ok(ExprNode {
        expr: Expr::Deref(DerefNode {
            deref_token,
            expr: Box::new(expr),
            span,
        }),
        span,
    });
}

pub fn parse_ref_expr(parser: &mut Parser) -> Result<ExprNode> {
    let ref_token = parser.consume(TokenType::Ampersand)?;
    let mut_token = parser.consume_if(TokenType::Keyword(TokenKeyword::Mut))?;

    let expr = parse_expr_layer_2(parser)?;

    let span = Span::from((ref_token.span, expr.span));

    return Ok(ExprNode {
        expr: Expr::Ref(RefNode {
            ref_token,
            mut_token,
            expr: Box::new(expr),
            span,
        }),
        span,
    });
}

pub fn parse_literal(parser: &mut Parser) -> Result<LiteralNode> {
    let token = parser
        .consume_current()
        .with_context(|| format!("Expected some expr to parse"))?;

    let literal = if let TokenType::Literal(literal) = token.token_type {
        literal
    } else {
        Err(ParseError::UnexpectedToken(file!(), line!(), token.clone()))?
    };

    match literal {
        TokenLiteral::Bool(is_true) => {
            return Ok(LiteralNode {
                span: token.span,
                literal: Literal::Bool(is_true),
            });
        }
        TokenLiteral::String => {
            return Ok(LiteralNode {
                span: token.span,
                literal: Literal::String(token.literal),
            });
        }
        _ => todo!("{literal:#?}"),
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

            let token = parser.current().ok();

            match token.map(|t| t.token_type) {
                Some(TokenType::DoubleColon) => {
                    let static_access = parse_static_access(parser, ident_lookup)?;

                    return Ok(ExprNode {
                        span: static_access.span,
                        expr: Expr::StaticAccess(static_access),
                    });
                }
                _ => {
                    return Ok(ExprNode {
                        span: ident_lookup.span,
                        expr: Expr::IdentLookup(ident_lookup),
                    })
                }
            }
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

        prev_comma = parser.consume_if(TokenType::Comma)?;
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

pub fn parse_static_access(parser: &mut Parser, lhs: IdentLookupNode) -> Result<StaticAccessNode> {
    let delim = parser.consume(TokenType::DoubleColon)?;

    let rhs = parse_expr_layer_3(parser)?;

    return Ok(StaticAccessNode {
        span: Span::from((lhs.span, rhs.span)),
        delim,
        lhs,
        rhs: Box::new(rhs),
    });
}
