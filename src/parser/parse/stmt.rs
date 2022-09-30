use super::*;

pub fn parse_statement(parser: &mut Parser) -> Result<StatementNode> {
    let token = parser
        .current()
        .with_context(|| format!("Expected some statement to parse"))?;

    let stmt = match token.token_type {
        TokenType::Identifier => parse_ident_stmt(parser)?,
        TokenType::Keyword(TokenKeyword::Const) | TokenType::Keyword(TokenKeyword::Let) =>
            parse_assign_stmt(parser)?,
        _ => {
            let expr = parse_expr(parser)?;

            StatementNode {
                span: expr.span,
                statement: Statement::Expr(expr),
            }
        }
    };

    require_newline(parser, stmt.span.to.line)?;

    return Ok(stmt);
}

pub fn parse_ident_stmt(parser: &mut Parser) -> Result<StatementNode> {
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

pub fn parse_assign_stmt(parser: &mut Parser) -> Result<StatementNode> {
    let (is_const, assign_token) = if parser.scan(&[TokenType::Keyword(TokenKeyword::Const)]) {
        (true, parser.consume(TokenType::Keyword(TokenKeyword::Const))?)
    } else {
        (false, parser.consume(TokenType::Keyword(TokenKeyword::Let))?)
    };

    let name = parser.consume(TokenType::Identifier)?;

    let mut span = Span::from((assign_token.span, name.span));

    let explicit_type = if parser.scan(&[TokenType::Colon]) {
        let colon = parser.consume(TokenType::Colon)?;
        let typ = parse_type(parser)?;

        span = Span::from((span, typ.span));

        Some((colon, typ))
    } else {
        None
    };

    let rhs_expr = if parser.scan(&[TokenType::Equals]) {
        let eq = parser.consume(TokenType::Equals)?;
        let expr = parse_expr(parser)?;

        span = Span::from((span, expr.span));

        Some((eq, expr))
    } else {
        None
    };

    return Ok(StatementNode {
        span,
        statement: Statement::Assign(AssignNode {
            span,
            is_const,
            assign_token,
            name,
            explicit_type,
            rhs_expr,
        }),
    });
}

