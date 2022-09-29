use super::*;

mod expr;
use expr::*;

mod fn_def;
use fn_def::*;

mod stmt;
use stmt::*;

mod r#type;
use r#type::*;

pub fn parse_file(parser: &mut Parser) -> Result<FerrumFileAst> {
    let mut ast = FerrumFileAst::new();

    while parser.index < parser.tokens.len() {
        let item = parse_item(parser)?;
        ast.items.push(item);
    }

    match parser.tokens.len() {
        0 => {}
        1 => ast.span = parser.tokens[0].span,
        n => ast.span = Span::from((parser.tokens[0].span, parser.tokens[n - 1].span)),
    }

    return Ok(ast);
}

fn require_newline(parser: &mut Parser, line: usize) -> Result {
    if let Some(next) = parser.current().ok() {
        if next.span.from.line == line {
            Err(ParseError::NotExpectedNewline(file!(), line!(), next))?;
        }
    }

    return Ok(());
}

fn parse_item(parser: &mut Parser) -> Result<ItemNode> {
    let public = parser.consume_if(TokenType::Keyword(TokenKeyword::Pub))?;

    let token = parser
        .current()
        .with_context(|| format!("Expected some item to parse"))?;
    
    let item = match token.token_type {
        TokenType::Keyword(TokenKeyword::Fn) => {
            let fn_definition = fn_def::parse_fn_def(parser, public)?;

            ItemNode {
                span: fn_definition.span,
                item: Item::FnDef(fn_definition),
            }
        }
        _ => {
            if let Some(public) = public {
                Err(ParseError::UnexpectedToken(file!(), line!(), public))?;
            };

            let statement = parse_statement(parser)?;

            ItemNode {
                span: statement.span,
                item: Item::Statement(statement),
            }
        }
    };

    require_newline(parser, item.span.to.line)?;

    return Ok(item);
}
