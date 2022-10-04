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

use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Translator {
    pub is_mod_root: bool,
    pub mods: HashSet<String>,
}

pub fn translate_file(
    translator: &mut Translator,
    fe_file: parser::ast::FerrumFileAst,
) -> Result<RustFileAst> {
    let mut rs_file = RustFileAst { items: vec![] };

    for item in fe_file.items {
        let items = translate_item(translator, item)?;

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
            let (use_node, mod_node) = translate_use(translator, use_node)?;

            let mut items = vec![];

            if let Some(use_node) = use_node {
                items.push(Item::Use(use_node));
            }

            if let Some(mod_node) = mod_node {
                items.push(Item::Mod(mod_node));
            }

            return Ok(items);
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
    use_node: parser::ast::UseNode,
) -> Result<(Option<Use>, Option<Mod>)> {
    if translator.is_mod_root {
        match use_node.use_pattern.use_pattern {
            parser::ast::InitUsePattern::Id(id) => {
                translator.mods.remove(&id.literal);

                return Ok((
                    None,
                    Some(Mod {
                        is_public: use_node.public.is_some(),
                        name: id.literal,
                    }),
                ));
            }
            p => {
                return Ok((
                    Some(Use {
                        is_public: use_node.public.is_some(),
                        use_pattern: translate_use_pattern(translator, p.into())?,
                    }),
                    None,
                ))
            }
        }
    }

    return Ok((
        Some(Use {
            is_public: use_node.public.is_some(),
            use_pattern: UsePattern::Path(UsePatternPath {
                parent: String::from("super"),
                rhs: Box::new(translate_use_pattern(
                    translator,
                    use_node.use_pattern.use_pattern.into(),
                )?),
            }),
        }),
        None,
    ));
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
        parser::ast::UsePattern::Destruct(destruct) => return Ok(UsePattern::Destruct(UsePatternDestruct {
            fields: destruct.patterns.take_values()
                .into_iter()
                .map(|p| translate_use_pattern(translator, p.use_pattern.into()))
                .collect::<Result<Vec<UsePattern>>>()?,
        })),
    }
}
