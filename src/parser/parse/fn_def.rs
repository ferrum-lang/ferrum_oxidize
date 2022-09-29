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

        params.push(prev_comma, FnDefParamNode {
            span,
            name,
            colon,
            param_type,
            default_value,
        });

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

    let open_brace = parser.consume(TokenType::OpenBrace)?;

    let mut body = vec![];

    while !parser.scan(&[TokenType::CloseBrace]) {
        let item = parse_item(parser)?;
        body.push(item);
    }

    let close_brace = parser.consume(TokenType::CloseBrace)?;

    let span = if let Some(pub_token) = &pub_token {
        Span::from((pub_token.span, close_brace.span))
    } else {
        Span::from((fn_token.span, close_brace.span))
    };

    return Ok(FnDefNode {
        pub_token,
        fn_token,
        name,
        open_paren,
        params,
        close_paren,
        return_type,
        open_brace,
        body,
        close_brace,
        span,
    });
}
