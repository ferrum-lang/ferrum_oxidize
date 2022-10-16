use super::*;

pub fn parse_use(parser: &mut Parser, public: Option<Token>) -> Result<UseNode> {
    let use_token = parser.consume(TokenType::Keyword(TokenKeyword::Use))?;

    let pattern_prefix = if parser.scan(&[TokenType::Tilde, TokenType::ForwardSlash]) {
        let tilde = parser.consume(TokenType::Tilde)?;
        let forward_slash = parser.consume(TokenType::ForwardSlash)?;

        Some(PatternPrefix::Root(RootPrefixNode {
            span: Span::from((tilde.span, forward_slash.span)),
            tilde,
            forward_slash,
        }))
    } else if parser.scan(&[TokenType::DoublePeriod, TokenType::ForwardSlash]) {
        let mut parent_dirs = vec![];
        let mut parent_span = None;

        while parser.scan(&[TokenType::DoublePeriod, TokenType::ForwardSlash]) {
            let double_period = parser.consume(TokenType::DoublePeriod)?;
            let forward_slash = parser.consume(TokenType::ForwardSlash)?;
            let span = Span::from((double_period.span, forward_slash.span));

            parent_dirs.push(ParentDirPrefixNode {
                span,
                double_period,
                forward_slash,
            });
            parent_span = Some(Span::from((parent_span, span)));
        }

        Some(PatternPrefix::Rel(RelPrefixNode {
            parent_dirs,
            span: parent_span.unwrap(),
        }))
    } else {
        None
    };

    let use_pattern = parse_init_use_pattern(parser)?;

    let span = if let Some(public) = &public {
        Span::from((public.span, use_pattern.span))
    } else {
        Span::from((use_token.span, use_pattern.span))
    };

    return Ok(UseNode {
        public,
        use_token,
        pattern_prefix,
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

fn parse_destruct_init_use_pattern(
    parser: &mut Parser,
) -> Result<UsePatternNode<DestructInitUsePattern>> {
    if let Some(self_) = parser.consume_if(TokenType::Keyword(TokenKeyword::Self_))? {
        return Ok(UsePatternNode {
            span: self_.span,
            use_pattern: DestructInitUsePattern::Self_(self_),
        });
    }

    if let Some(wild) = parser.consume_if(TokenType::Asterisk)? {
        return Ok(UsePatternNode {
            span: wild.span,
            use_pattern: DestructInitUsePattern::Wild(wild),
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
