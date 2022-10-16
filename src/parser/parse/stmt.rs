use super::*;

pub fn parse_statement(parser: &mut Parser) -> Result<StatementNode> {
    let token = parser
        .current()
        .with_context(|| format!("Expected some statement to parse"))?;

    let stmt = match token.token_type {
        // TokenType::Keyword(TokenKeyword::Return) => parse_return_stmt(parser)?,
        TokenType::Keyword(TokenKeyword::Const) | TokenType::Keyword(TokenKeyword::Let) =>
            parse_assign_decl(parser)?,
        _ => {
            let expr = parse_expr(parser)?;

            StatementNode {
                span: expr.span,
                statement: Statement::Expr(expr),
            }
        }
    };

    // TODO: Check for 2nd part of statments (ie. =, !=, +=, ...)

    require_newline(parser, stmt.span.to.line)?;

    return Ok(stmt);
}

pub fn parse_assign_decl(parser: &mut Parser) -> Result<StatementNode> {
    let (is_const, decl_token) = if parser.scan(&[TokenType::Keyword(TokenKeyword::Const)]) {
        (true, parser.consume(TokenType::Keyword(TokenKeyword::Const))?)
    } else {
        (false, parser.consume(TokenType::Keyword(TokenKeyword::Let))?)
    };

    let assign_pattern = parse_assign_pattern(parser)?;

    let mut span = Span::from((decl_token.span, assign_pattern.span));

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
        statement: Statement::Decl(DeclarationNode {
            span,
            is_const,
            decl_token,
            assign_pattern,
            explicit_type,
            rhs_expr,
        }),
    });
}

pub fn parse_assign_pattern(parser: &mut Parser) -> Result<AssignPatternNode> {
    let token = parser
        .consume_current()
        .with_context(|| format!("Expected some assign pattern to parse"))?;

    match token.token_type {
        TokenType::Identifier => return Ok(AssignPatternNode {
            span: token.span,
            assign_pattern: AssignPattern::Id(token),
        }),
        _ => todo!(),
    }
}

