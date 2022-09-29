use super::*;

mod stmt;
use stmt::*;

mod expr;
use expr::*;

pub fn parse_file(parser: &mut Parser) -> Result<FerrumFileAst> {
    let mut ast = FerrumFileAst::new();

    while parser.index < parser.tokens.len() {
        let item = parse_item(parser)?;

        if let Some(next) = parser.current().ok() {
            if next.span.from.line == item.span.to.line {
                Err(ParseError::NotExpectedNewline(file!(), line!(), next))?;
            }
        }

        ast.items.push(item);
    }

    match parser.tokens.len() {
        0 => {}
        1 => ast.span = parser.tokens[0].span,
        n => ast.span = Span::from((parser.tokens[0].span, parser.tokens[n - 1].span)),
    }

    return Ok(ast);
}

fn parse_item(parser: &mut Parser) -> Result<ItemNode> {
    let token = parser
        .current()
        .with_context(|| format!("Expected some item to parse"))?;

    match token.token_type {
        _ => {
            let statement = parse_statement(parser)?;

            return Ok(ItemNode {
                span: statement.span,
                item: Item::Statement(statement),
            });
        }
    }
}

