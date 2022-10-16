use super::*;

mod expr;
use expr::*;

mod fn_def;
use fn_def::*;

mod stmt;
use stmt::*;

mod r#type;
use r#type::*;

use crate::parser;
use crate::Result;

#[derive(Debug, Clone)]
pub struct Translator {
    pub node: FeShared<parser::ast::FerrumModNode>,
    scope_stack: Vec<parser::ast::ScopeTable>,
}

impl Translator {
    pub fn new(node: FeShared<parser::ast::FerrumModNode>) -> Self {
        let mut scope_stack = vec![];

        match &node.file {
            parser::ast::FerrumModNodeFile::File(file) => {
                scope_stack.push(file.scope.clone());
            }
            _ => {}
        }

        return Self { node, scope_stack };
    }

    pub fn find_in_scope(&self, name: &str) -> Option<parser::ast::ScopeRefNode> {
        for scope in self.scope_stack.iter().rev() {
            if let Some(scope_ref) = scope.get(name) {
                return Some(scope_ref.clone());
            }
        }

        return None;
    }
}

pub fn translate_file(
    translator: &mut Translator,
    fe_file: &parser::ast::FerrumFileAst,
) -> Result<RustFileAst> {
    let mut rs_file = RustFileAst { items: vec![] };

    for item in &fe_file.items {
        let items = translate_item(translator, FeShared::get(&item).clone())?;

        rs_file.items.extend(items);
    }

    return Ok(rs_file);
}

pub fn translate_item(
    translator: &mut Translator,
    item: parser::ast::ItemNode,
) -> Result<Vec<Item>> {
    match item.item {
        parser::ast::Item::Use(use_node) => {
            let use_node = translate_use(translator, use_node)?;
            return Ok(vec![Item::Use(use_node)]);
        }
        parser::ast::Item::FnDef(fn_def) => {
            let fn_def = translate_fn_def(translator, fn_def)?;
            return Ok(vec![Item::FnDef(fn_def)]);
        }
        item => todo!("Cannot translate item: {item:#?}"),
    }
}

pub fn translate_use(
    translator: &mut Translator,
    mut use_node: parser::ast::UseNode,
) -> Result<Use> {
    use crate::lexer::token::TokenType;
    use crate::span::Span;

    match use_node.pattern_prefix {
        Some(parser::ast::PatternPrefix::Root(_)) => {
            use_node.use_pattern = parser::ast::UsePatternNode {
                use_pattern: parser::ast::InitUsePattern::Path(parser::ast::UsePatternPath {
                    parent_name: parser::ast::Token::new(
                        TokenType::Identifier,
                        String::from("crate"),
                        Span::from((0, 0)),
                    ),
                    delim: parser::ast::Token::new(
                        TokenType::DoubleColon,
                        "::",
                        Span::from((0, 0)),
                    ),
                    rhs: Box::new(parser::ast::UsePatternNode {
                        use_pattern: parser::ast::UsePattern::from(
                            use_node.use_pattern.use_pattern,
                        ),
                        span: Span::from((0, 0)),
                    }),
                }),
                span: Span::from((0, 0)),
            };
        }
        Some(parser::ast::PatternPrefix::Rel(rel)) => {
            for _ in rel.parent_dirs {
                use_node.use_pattern = parser::ast::UsePatternNode {
                    use_pattern: parser::ast::InitUsePattern::Path(parser::ast::UsePatternPath {
                        parent_name: parser::ast::Token::new(
                            TokenType::Identifier,
                            String::from("super"),
                            Span::from((0, 0)),
                        ),
                        delim: parser::ast::Token::new(
                            TokenType::DoubleColon,
                            "::",
                            Span::from((0, 0)),
                        ),
                        rhs: Box::new(parser::ast::UsePatternNode {
                            use_pattern: parser::ast::UsePattern::from(
                                use_node.use_pattern.use_pattern,
                            ),
                            span: Span::from((0, 0)),
                        }),
                    }),
                    span: Span::from((0, 0)),
                };
            }
        }
        None => {}
    }

    return Ok(Use {
        is_public: use_node.public.is_some(),
        use_pattern: translate_use_pattern(translator, use_node.use_pattern.use_pattern.into())?,
    });
}

fn translate_use_pattern(
    translator: &mut Translator,
    use_pattern: parser::ast::UsePattern,
) -> Result<UsePattern> {
    match use_pattern {
        parser::ast::UsePattern::Id(id) => return Ok(UsePattern::Id(id.literal)),
        parser::ast::UsePattern::Path(path) => {
            return Ok(UsePattern::Path(UsePatternPath {
                parent: path.parent_name.literal,
                rhs: Box::new(translate_use_pattern(translator, path.rhs.use_pattern)?),
            }))
        }
        parser::ast::UsePattern::Wild(_) => return Ok(UsePattern::Wild),
        parser::ast::UsePattern::Destruct(destruct) => {
            return Ok(UsePattern::Destruct(UsePatternDestruct {
                fields: destruct
                    .patterns
                    .take_values()
                    .into_iter()
                    .map(|p| translate_use_pattern(translator, p.use_pattern.into()))
                    .collect::<Result<Vec<UsePattern>>>()?,
            }))
        }
    }
}
