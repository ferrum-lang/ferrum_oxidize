use super::*;

pub fn parse_statement(parser: &mut Parser) -> Result<StatementNode> {
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


