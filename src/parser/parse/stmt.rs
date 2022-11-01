use super::*;

pub fn parse_statement(parser: &mut Parser) -> Result<StatementNode> {
    let token = parser
        .current()
        .with_context(|| format!("Expected some statement to parse"))?;

    let mut stmt = match token.token_type {
        // TokenType::Keyword(TokenKeyword::Return) => parse_return_stmt(parser)?,
        TokenType::Keyword(TokenKeyword::Pass) => {
            let pass = parser.consume(TokenType::Keyword(TokenKeyword::Pass))?;
            StatementNode {
                span: pass.span,
                statement: Statement::Pass,
            }
        },
        TokenType::Keyword(TokenKeyword::Do) => {
            let do_node = parse_do(parser)?;
            StatementNode {
                span: do_node.span,
                statement: Statement::Do(do_node),
            }
        },
        TokenType::Keyword(TokenKeyword::Const) | TokenType::Keyword(TokenKeyword::Let) =>
            parse_decl(parser)?,
        _ => {
            let expr = parse_expr(parser)?;

            StatementNode {
                span: expr.span,
                statement: Statement::Expr(expr),
            }
        }
    };

    let token = parser.current().ok();

    match token.map(|t| t.token_type) {
        Some(TokenType::Equals) => {
            stmt = parse_assign_stmt(parser, stmt)?;
        }
        _ => {},
    }

    parser.expect_newline(stmt.span.last_line())?;

    return Ok(stmt);
}

pub fn parse_decl(parser: &mut Parser) -> Result<StatementNode> {
    let (is_const, decl_token) = if let Some(token) = parser.consume_if(TokenType::Keyword(TokenKeyword::Const))? {
        (true, token)
    } else {
        (false, parser.consume(TokenType::Keyword(TokenKeyword::Let))?)
    };

    let decl_pattern = parse_decl_pattern(parser)?;

    let mut span = Span::from((decl_token.span, decl_pattern.span));

    let explicit_type = if parser.scan(&[TokenType::Colon]) {
        let colon = parser.consume(TokenType::Colon)?;
        let typ = parse_type(parser)?;

        span = Span::from((span, typ.span));

        Some((colon, typ))
    } else {
        None
    };

    return Ok(StatementNode {
        span,
        statement: Statement::Decl(DeclarationNode {
            span,
            is_const,
            decl_token,
            decl_pattern,
            explicit_type,
        }),
    });
}

pub fn parse_decl_pattern(parser: &mut Parser) -> Result<DeclPatternNode> {
    let token = parser
        .consume_current()
        .with_context(|| format!("Expected some decl pattern to parse"))?;

    match token.token_type {
        TokenType::Identifier => return Ok(DeclPatternNode {
            span: token.span,
            decl_pattern: DeclPattern::Id(token),
        }),
        _ => todo!(),
    }
}

fn parse_do(parser: &mut Parser) -> Result<DoNode> {
    let token = parser.consume(TokenType::Keyword(TokenKeyword::Do))?;

    let next_token_line = parser
        .current()
        .with_context(|| format!("Expected some statement to parse"))?
        .first_line();

    if token.last_line() == next_token_line {
        let stmt = parse_statement(parser)?;
        let if_token = parser.consume(TokenType::Keyword(TokenKeyword::If))?;
        todo!();
    } else {
        let mut stmts = vec![];

        while !parser.scan(&[TokenType::Semicolon]) {
            let stmt = parse_statement(parser)?;

            parser.expect_newline(stmt.span.last_line())?;

            stmts.push(stmt);
        }

        let close_semicolon = parser.consume(TokenType::Semicolon)?;

        return Ok(DoNode {
            span: Span::from((token.span, close_semicolon.span)),
            stmts,
            close_semicolon,
        });
    };
}

fn parse_assign_stmt(parser: &mut Parser, lhs: StatementNode) -> Result<StatementNode> {
    let assign_token = parser
        .consume_current()
        .with_context(|| format!("Expected some assign token to parse"))?;

    match &assign_token.token_type {
        TokenType::Equals => {},
        _ => todo!(),
    }

    let rhs = parse_expr(parser)?;

    let span = Span::from((lhs.span, rhs.span));

    return Ok(StatementNode {
        statement: Statement::Assign(AssignNode {
            lhs: Box::new(lhs),
            assign: assign_token,
            rhs,
            span,
        }),
        span,
    });
}

