use super::*;

pub fn parse_use(parser: &mut Parser, public: Option<Token>) -> Result<UseNode> {
    let use_token = parser.consume(TokenType::Keyword(TokenKeyword::Use))?;

    let use_pattern = parse_init_use_pattern(parser)?;

    let span = if let Some(public) = &public {
        Span::from((public.span, use_pattern.span))
    } else {
        Span::from((use_token.span, use_pattern.span))
    };

    return Ok(UseNode {
        public,
        use_token,
        use_pattern,
        span,
    });
}

fn parse_init_use_pattern(parser: &mut Parser) -> Result<UsePatternNode<InitUsePattern>> {
    let token = parser
        .consume(TokenType::Identifier)
        .with_context(|| "Expected some use pattern to parse")?;

    if let Some(delim) = parser.consume_if(TokenType::DoubleColon)? {
        let rhs = parse_use_pattern(parser)?;

        return Ok(UsePatternNode {
            span: Span::from((token.span, rhs.span)),
            use_pattern: InitUsePattern::Path(UsePatternPath {
                parent_name: token,
                delim,
                rhs: Box::new(rhs),
            }),
        });
    } else {
        return Ok(UsePatternNode {
            span: token.span,
            use_pattern: InitUsePattern::Id(token),
        });
    }
}

fn parse_use_pattern(parser: &mut Parser) -> Result<UsePatternNode> {
    let token = parser
        .consume_current()
        .with_context(|| "Expected some use pattern to parse")?;

    match token.token_type {
        TokenType::Identifier => {
            if let Some(delim) = parser.consume_if(TokenType::DoubleColon)? {
                let rhs = parse_use_pattern(parser)?;

                return Ok(UsePatternNode {
                    span: Span::from((token.span, rhs.span)),
                    use_pattern: UsePattern::Path(UsePatternPath {
                        parent_name: token,
                        delim,
                        rhs: Box::new(rhs),
                    }),
                });
            } else {
                return Ok(UsePatternNode {
                    span: token.span,
                    use_pattern: UsePattern::Id(token),
                });
            }
        }
        TokenType::Asterisk => {
            return Ok(UsePatternNode {
                span: token.span,
                use_pattern: UsePattern::Wild(token),
            })
        }
        TokenType::OpenBrace => {
            let mut patterns = Punctuated::new();
            let mut prev_comma = None;

            while !parser.scan(&[TokenType::CloseBrace]) {
                let pattern = Box::new(parse_destruct_init_use_pattern(parser)?);

                if let Some(prev_comma) = prev_comma {
                    patterns.push_delim(prev_comma, pattern);
                } else {
                    patterns.push_first(pattern);
                }

                prev_comma = parser.consume_if(TokenType::Comma)?;
            }

            let close_brace = parser.consume(TokenType::CloseBrace)?;

            return Ok(UsePatternNode {
                span: Span::from((token.span, close_brace.span)),
                use_pattern: UsePattern::Destruct(UsePatternDestruct {
                    open_brace: token,
                    patterns,
                    close_brace,
                }),
            });
        }
        _ => todo!("{token:#?}"),
    }
}

fn parse_destruct_init_use_pattern(parser: &mut Parser) -> Result<UsePatternNode<DestructInitUsePattern>> {
    if let Some(self_) = parser.consume_if(TokenType::Keyword(TokenKeyword::Self_))? {
        return Ok(UsePatternNode {
            span: self_.span,
            use_pattern: DestructInitUsePattern::Self_(self_),
        });
    }

    let token = parser
        .consume(TokenType::Identifier)
        .with_context(|| "Expected some use pattern to parse")?;

    if let Some(delim) = parser.consume_if(TokenType::DoubleColon)? {
        let rhs = parse_use_pattern(parser)?;

        return Ok(UsePatternNode {
            span: Span::from((token.span, rhs.span)),
            use_pattern: DestructInitUsePattern::Path(UsePatternPath {
                parent_name: token,
                delim,
                rhs: Box::new(rhs),
            }),
        });
    } else {
        return Ok(UsePatternNode {
            span: token.span,
            use_pattern: DestructInitUsePattern::Id(token),
        });
    }
}
