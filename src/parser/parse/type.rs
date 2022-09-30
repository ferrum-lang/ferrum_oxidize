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
        _ => todo!(),
    }
}
