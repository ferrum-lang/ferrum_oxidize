mod error;
mod prep;

pub mod ast;
use ast::*;

pub mod translate;
use translate::*;

mod runtime;
use runtime::*;

pub use error::TranslateError;

use crate::parser;
use crate::Result;

use std::collections::HashMap;

use ferrum_runtime::prelude::FeShared;

pub fn translate_to_rust(
    mut root_node: FeShared<parser::ast::FerrumModNode>,
) -> Result<RustProject> {
    prep::prepare_fe_ast_for_translation(&mut root_node)?;

    let src = translate_to_rust_node(FeShared::share(&root_node), true)?;

    match src.file {
        RustModNodeFile::Dir(mut nodes) => match nodes.remove("main").unwrap().file {
            RustModNodeFile::FerrumFile(main_file) => {
                return Ok(RustProject {
                    main_file,
                    siblings: nodes,
                })
            }
            _ => todo!(),
        },
        _ => todo!(),
    };
}

fn translate_to_rust_node(
    fe_node: FeShared<parser::ast::FerrumModNode>,
    is_entry: bool,
) -> Result<RustModNode> {
    let mut translator = Translator::new(FeShared::share(&fe_node));

    match &fe_node.file {
        parser::ast::FerrumModNodeFile::Dir(fe_nodes) => {
            let mut nodes = HashMap::new();

            for (_, fe_node) in fe_nodes {
                let node = translate_to_rust_node(FeShared::share(fe_node), false)?;
                nodes.insert(node.name.clone(), node);
            }

            return Ok(RustModNode {
                name: fe_node.name.clone(),
                file: RustModNodeFile::Dir(nodes),
            });
        }
        parser::ast::FerrumModNodeFile::File(fe_file) => {
            match fe_file.file_type {
                parser::ast::FileType::Ferrum => {},
                parser::ast::FileType::LocalRustBind => {
                    let rs_path = fe_node.path.with_extension("rs");

                    return Ok(RustModNode {
                        name: fe_node.name.clone(),
                        file: RustModNodeFile::RustFile(rs_path)
                    });
                },
            }

            let mut file = translate_file(&mut translator, fe_file)?;

            if is_entry || fe_node.name.as_str() == "_pkg" {
                let mut mods_to_add = HashMap::new();

                file.items = file
                    .items
                    .into_iter()
                    .filter_map(|item| match item {
                        Item::Use(Use {
                            is_public,
                            ref use_pattern,
                        }) => match use_pattern {
                            UsePattern::Id(name) if fe_node.sibling_refs.contains_key(name) => {
                                mods_to_add.insert(
                                    name.clone(),
                                    Mod {
                                        is_public,
                                        name: name.clone(),
                                    },
                                );
                                return None;
                            }
                            UsePattern::Path(UsePatternPath { parent, rhs })
                                if fe_node.sibling_refs.contains_key(parent) =>
                            {
                                match &**rhs {
                                    UsePattern::Destruct(UsePatternDestruct { fields }) => {
                                        return Some(Item::Use(Use {
                                            is_public,
                                            use_pattern: UsePattern::Path(UsePatternPath {
                                                parent: parent.clone(),
                                                rhs: Box::new(UsePattern::Destruct(
                                                    UsePatternDestruct {
                                                        fields: fields
                                                            .clone()
                                                            .into_iter()
                                                            .filter(|field| match field {
                                                                UsePattern::Id(name)
                                                                    if name.as_str() == "self" =>
                                                                {
                                                                    mods_to_add.insert(
                                                                        parent.clone(),
                                                                        Mod {
                                                                            is_public,
                                                                            name: parent.clone(),
                                                                        },
                                                                    );
                                                                    return false;
                                                                }
                                                                _ => true,
                                                            })
                                                            .collect(),
                                                    },
                                                )),
                                            }),
                                        }))
                                    }
                                    _ => Some(item),
                                }
                            }
                            _ => Some(item),
                        },
                        _ => Some(item),
                    })
                    .collect();

                for (sibling, _) in &fe_node.sibling_refs {
                    let mod_to_add = mods_to_add.remove(sibling).unwrap_or_else(|| Mod {
                        is_public: false,
                        name: sibling.clone(),
                    });

                    file.items.insert(0, Item::Mod(mod_to_add));
                }
            } else {
                file.items = file
                    .items
                    .into_iter()
                    .map(|item| match item {
                        Item::Use(Use {
                            is_public,
                            use_pattern: UsePattern::Path(UsePatternPath { parent, rhs }),
                        }) if parent != "crate" => Item::Use(Use {
                            is_public,
                            use_pattern: UsePattern::Path(UsePatternPath {
                                parent: String::from("super"),
                                rhs: Box::new(UsePattern::Path(UsePatternPath { parent, rhs })),
                            }),
                        }),
                        _ => item,
                    })
                    .collect();
            }

            if is_entry {
                let mut nodes = HashMap::new();

                nodes.insert(
                    String::from("main"),
                    RustModNode {
                        name: String::from("main"),
                        file: RustModNodeFile::FerrumFile(file),
                    },
                );

                for (_, sibling_ref) in &fe_node.sibling_refs {
                    let node = translate_to_rust_node(FeShared::share(sibling_ref), false)?;
                    nodes.insert(node.name.clone(), node);
                }

                return Ok(RustModNode {
                    name: String::from("src"),
                    file: RustModNodeFile::Dir(nodes),
                });
            }

            return Ok(RustModNode {
                name: if fe_node.name.as_str() == "_pkg" {
                    String::from("mod")
                } else {
                    fe_node.name.clone()
                },
                file: RustModNodeFile::FerrumFile(file),
            });
        }
    }
}
