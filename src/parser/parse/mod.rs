use super::*;

mod expr;
use expr::*;

mod fn_def;
use fn_def::*;

mod stmt;
use stmt::*;

mod r#type;
use r#type::*;

mod r#use;
use r#use::*;

pub fn parse_file(parser: &mut Parser) -> Result<FerrumFileAst> {
    let mut ast = FerrumFileAst::new(if parser.require_impl {
        FileType::Ferrum
    } else {
        FileType::LocalRustBind
    });

    ast.items = parse_items_while(parser, |parser| parser.index < parser.tokens.len())?;

    match parser.tokens.len() {
        0 => {}
        1 => ast.span = parser.tokens[0].span,
        n => ast.span = Span::from((parser.tokens[0].span, parser.tokens[n - 1].span)),
    }

    return Ok(ast);
}

pub fn parse_items_while(
    parser: &mut Parser,
    condition: impl Fn(&mut Parser) -> bool,
) -> Result<Vec<FeShared<ItemNode>>> {
    let mut items = vec![];

    let mut allow_uses = true;

    while condition(parser) {
        let item = parse_item(parser)?;

        match item.item {
            Item::Use(_) if allow_uses => {}
            Item::Use(_) if !allow_uses => {
                dbg!(&item);
                todo!("Error! Uses must be at the top of the scoped block");
            }
            _ if allow_uses => {
                allow_uses = false;
            }
            _ => {}
        }

        items.push(FeShared::new(item));
    }

    return Ok(items);
}

fn parse_item(parser: &mut Parser) -> Result<ItemNode> {
    let public = parser.consume_if(TokenType::Keyword(TokenKeyword::Pub))?;

    let token = parser
        .current()
        .with_context(|| format!("Expected some item to parse"))?;

    let item = match token.token_type {
        TokenType::Keyword(TokenKeyword::Use) => {
            let use_node = parse_use(parser, public)?;

            ItemNode {
                span: use_node.span,
                item: Item::Use(use_node),
            }
        }
        TokenType::Keyword(TokenKeyword::Fn) => {
            let fn_definition = parse_fn_def(parser, public)?;

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

    parser.expect_newline(item.span.last_line())?;

    return Ok(item);
}
