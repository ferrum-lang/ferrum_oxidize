use super::*;

pub fn parse_type(parser: &mut Parser) -> Result<TypeNode> {
    let token = parser
        .consume_current()
        .with_context(|| format!("Expected some type to parse"))?;

    match token.token_type {
        TokenType::Primitive(TokenPrimitive::String) => {
            return Ok(TypeNode {
                span: token.span,
                typ: Type::String(token),
            })
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
        _ => todo!(),
    }
}
