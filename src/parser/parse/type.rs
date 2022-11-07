use super::*;

pub fn parse_type(parser: &mut Parser) -> Result<TypeNode> {
    let token = parser
        .consume_current()
        .with_context(|| format!("Expected some type to parse"))?;

    match token.token_type {
        TokenType::QuestionMark => {
            let inner = Box::new(parse_type(parser)?);
            let span = Span::from((token.span, inner.span));

            return Ok(TypeNode {
                span,
                typ: Type::Optional(OptionalNode {
                    question_mark: token,
                    of: inner,
                    span,
                }),
            });
        }
        TokenType::Primitive(TokenPrimitive::String) => {
            return Ok(TypeNode {
                span: token.span,
                typ: Type::String(token),
            })
        }
        TokenType::Primitive(TokenPrimitive::Bool) => {
            return Ok(TypeNode {
                span: token.span,
                typ: Type::Bool(token),
            });
        }
        TokenType::Ampersand => {
            if let Some(mut_token) = parser.consume_if(TokenType::Keyword(TokenKeyword::Mut))? {
                return Ok(TypeNode {
                    span: token.span,
                    typ: Type::MutRef(MutRefNode {
                        span: token.span,
                        ref_token: token,
                        mut_token,
                        of: Box::new(parse_type(parser)?),
                    }),
                });
            } else {
                return Ok(TypeNode {
                    span: token.span,
                    typ: Type::SharedRef(SharedRefNode {
                        span: token.span,
                        ref_token: token,
                        of: Box::new(parse_type(parser)?),
                    }),
                });
            }
        }
        TokenType::DoubleAmpersand => {
            let span1 = Span::from((token.span.from, token.span.from));
            let span2 = Span::from((token.span.to, token.span.to));

            return Ok(TypeNode {
                span: span1,
                typ: Type::SharedRef(SharedRefNode {
                    span: span1,
                    ref_token: Token {
                        span: span1,
                        literal: String::from("&"),
                        token_type: TokenType::Ampersand,
                    },
                    of: Box::new(TypeNode {
                        span: span2,
                        typ: if let Some(mut_token) =
                            parser.consume_if(TokenType::Keyword(TokenKeyword::Mut))?
                        {
                            Type::MutRef(MutRefNode {
                                span: Span::from((span2, mut_token.span)),
                                ref_token: Token {
                                    span: span2,
                                    literal: String::from("&"),
                                    token_type: TokenType::Ampersand,
                                },
                                mut_token,
                                of: Box::new(parse_type(parser)?),
                            })
                        } else {
                            Type::SharedRef(SharedRefNode {
                                span: span2,
                                ref_token: Token {
                                    span: span2,
                                    literal: String::from("&"),
                                    token_type: TokenType::Ampersand,
                                },
                                of: Box::new(parse_type(parser)?),
                            })
                        },
                    }),
                }),
            });
        }
        _ => todo!("{token:#?}"),
    }
}
