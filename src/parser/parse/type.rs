use super::*;

pub fn parse_type(parser: &mut Parser) -> Result<TypeNode> {
    let token = parser
        .current()
        .with_context(|| format!("Expected some type to parse"))?;

    match token.token_type {
        TokenType::Primitive(TokenPrimitive::String) => {
            parser.index += 1;

            return Ok(TypeNode {
                span: token.span,
                typ: Type::String(token),
            });
        }
        TokenType::Ampersand => {
            parser.index += 1;

            if parser.scan(&[TokenType::Keyword(TokenKeyword::Mut)]) {
                parser.index += 1;

                return Ok(TypeNode {
                    span: token.span,
                    typ: Type::MutRef(Box::new(parse_type(parser)?)),
                });
            } else {
                return Ok(TypeNode {
                    span: token.span,
                    typ: Type::SharedRef(Box::new(parse_type(parser)?)),
                });
            }
        }
        _ => todo!(),
    }
}
