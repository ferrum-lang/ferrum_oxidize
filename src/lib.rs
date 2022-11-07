#![allow(dead_code)]
#![allow(unused_variables)]

mod analysis;
mod cargo;
mod config;
mod error;
mod generator;
mod lexer;
mod parser;
mod punctuated;
mod result;
mod span;
mod target;
mod translator;

pub use config::Config;
pub use error::OxidizeError;
pub use result::Result;
pub use target::Target;

#[macro_use]
extern crate lazy_static;

use parser::ast::{self, FerrumFileAst, FerrumModNode, FerrumModNodeFile};

use translator::ast::RustProject;

use cargo::project::CargoProject;

use std::{collections::HashMap, fs, path::PathBuf};

use ferrum_runtime::prelude::FeShared;

#[derive(Debug, Clone)]
pub struct FerrumProject {
    pub entry_file: PathBuf,
    pub build_dir: PathBuf,
    pub out_file: PathBuf,
    pub target: Target,
}

pub fn build_project(cfg: Config) -> Result<FerrumProject> {
    let entry_file = config::determine_entry_file(cfg.entry_file)?;
    // dbg!(&entry_file);

    let build_dir = config::determine_build_dir(cfg.build_dir)?;
    // dbg!(&build_dir);

    let out_file = config::determine_out_file(cfg.out_file, &entry_file)?;
    // dbg!(&out_file);

    let target = config::determine_target(cfg.target)?;
    // dbg!(&target);

    let cargo_project = build_to_cargo_project(entry_file.clone(), build_dir.clone())?;

    cargo_build(cargo_project, target.clone(), out_file.clone())?;

    return Ok(FerrumProject {
        entry_file,
        build_dir,
        out_file,
        target,
    });
}

pub fn build_to_cargo_project(entry_file: PathBuf, build_dir: PathBuf) -> Result<CargoProject> {
    let ferrum_ast = compile_to_ferrum_project_ast(entry_file.clone())?;

    let rust_ast = translate_to_rust_ast(ferrum_ast)?;

    let cargo_project = generate_cargo_project(rust_ast, build_dir)?;

    return Ok(cargo_project);
}

enum FileType {
    Dir,
    Ferrum,
    // Rust,
    RustBinding,
}

pub fn compile_to_ferrum_project_ast(entry_file: PathBuf) -> Result<FeShared<FerrumModNode>> {
    let name = if let Some(filename) = entry_file.file_stem() {
        filename.to_string_lossy().to_string()
    } else {
        todo!("Invalid file: {:?}", entry_file);
    };

    let mut root_node = FeShared::new(compile_to_ferrum_mod_node(entry_file, name)?);

    fn rec_compile_file_into_node(
        file_type: FileType,
        filepath: PathBuf,
        name: String,
    ) -> Result<FeShared<FerrumModNode>> {
        match file_type {
            FileType::Ferrum => {
                let node = compile_to_ferrum_mod_node(filepath, name)?;
                return Ok(FeShared::new(node));
            }
            // FileType::Rust => {
            //     let node = compile_to_rust_mod_node(filepath, name)?;
            //     return Ok(FeShared::new(node));
            // }
            FileType::RustBinding => {
                let node = compile_to_rust_bindings_node(filepath, name)?;
                return Ok(FeShared::new(node));
            }
            FileType::Dir => {
                let mut dir_node = FeShared::new(FerrumModNode::new(
                    name,
                    filepath.clone(),
                    FerrumModNodeFile::Dir(HashMap::new()),
                    parser::ast::FileType::Ferrum,
                ));

                let mut nodes = HashMap::new();
                let mut seen_pkg = false;

                for file in filepath.read_dir()? {
                    let file = file?;
                    let filename = file.file_name().to_string_lossy().to_string();

                    let filepath = filepath.join(&filename);

                    if file.file_type()?.is_dir() {
                        let mut node =
                            rec_compile_file_into_node(FileType::Dir, filepath, filename.clone())?;
                        node.parent_ref = Some(Box::new(FeShared::share(&dir_node)));
                        nodes.insert(filename, node);
                    } else {
                        let (name, file_type) = if filename.ends_with(".fe") {
                            (filename[..filename.len() - 3].to_string(), FileType::Ferrum)
                        // } else if filename.ends_with(".rs") {
                        //     (filename[..filename.len() - 3].to_string(), FileType::Rust)
                        } else if filename.ends_with(".rs-fe") {
                            (
                                filename[..filename.len() - 6].to_string(),
                                FileType::RustBinding,
                            )
                        } else {
                            continue;
                        };

                        if name.as_str() == "_pkg" {
                            if let FileType::Ferrum = file_type {
                                seen_pkg = true;
                            } else {
                                todo!("Invalid file: {filepath:?}");
                            }
                        }

                        let mut node =
                            rec_compile_file_into_node(file_type, filepath, name.clone())?;

                        node.parent_ref = Some(Box::new(FeShared::share(&dir_node)));
                        nodes.insert(name, node);
                    }
                }

                if !seen_pkg {
                    let pkg_name = String::from("_pkg");

                    let mut pkg_file = FerrumFileAst::new(parser::ast::FileType::Ferrum);
                    for node_name in nodes.keys() {
                        pkg_file.items.push(FeShared::new(ast::ItemNode {
                            item: ast::Item::Use(ast::UseNode {
                                extern_type: None,
                                pattern_prefix: None,
                                public: Some(ast::Token::new(
                                    lexer::token::TokenType::Keyword(
                                        lexer::token::TokenKeyword::Pub,
                                    ),
                                    "pub",
                                    span::Span::from((0, 0)),
                                )),
                                use_token: ast::Token::new(
                                    lexer::token::TokenType::Keyword(
                                        lexer::token::TokenKeyword::Use,
                                    ),
                                    "use",
                                    span::Span::from((0, 0)),
                                ),
                                use_pattern: ast::UsePatternNode {
                                    use_pattern: ast::InitUsePattern::Id(ast::Token::new(
                                        lexer::token::TokenType::Identifier,
                                        node_name,
                                        span::Span::from((0, 0)),
                                    )),
                                    span: span::Span::from((0, 0)),
                                },
                                span: span::Span::from((0, 0)),
                            }),
                            span: span::Span::from((0, 0)),
                        }));
                    }

                    let pkg_node = FerrumModNode::new(
                        pkg_name.clone(),
                        filepath.join("_pkg.fe"),
                        FerrumModNodeFile::File(pkg_file),
                        parser::ast::FileType::Ferrum,
                    );

                    nodes.insert(pkg_name, FeShared::new(pkg_node));
                }

                for (name1, node1) in &nodes {
                    let mut node1 = FeShared::share(node1);

                    for (name2, node2) in &nodes {
                        if name1 != name2 {
                            node1
                                .sibling_refs
                                .insert(name2.clone(), FeShared::share(node2));
                        }
                    }
                }

                dir_node.file = FerrumModNodeFile::Dir(nodes);

                return Ok(dir_node);
            }
        }
    }

    if let Some(dir) = FeShared::share(&root_node).path.parent() {
        for file in dir.read_dir()? {
            let file = file?;
            let filename = file.file_name().to_string_lossy().to_string();

            if filename.starts_with(".") {
                continue;
            }

            let filepath = dir.join(&filename);

            if file.file_type()?.is_dir() {
                let mut node =
                    rec_compile_file_into_node(FileType::Dir, filepath, filename.clone())?;
                node.sibling_refs
                    .insert(root_node.name.clone(), FeShared::share(&root_node));
                root_node.sibling_refs.insert(filename, node);
            } else {
                let (name, file_type) = if filename.ends_with(".fe") {
                    (filename[..filename.len() - 3].to_string(), FileType::Ferrum)
                // } else if filename.ends_with(".rs") {
                //     (filename[..filename.len() - 3].to_string(), FileType::Rust)
                } else if filename.ends_with(".rs-fe") {
                    (
                        filename[..filename.len() - 6].to_string(),
                        FileType::RustBinding,
                    )
                } else {
                    continue;
                };

                if name != root_node.name {
                    let mut node = rec_compile_file_into_node(file_type, filepath, name.clone())?;

                    node.sibling_refs
                        .insert(root_node.name.clone(), FeShared::share(&root_node));
                    root_node.sibling_refs.insert(name, node);
                }
            }
        }

        for (name1, node1) in &root_node.sibling_refs {
            let mut node1 = FeShared::share(node1);

            for (name2, node2) in &root_node.sibling_refs {
                if name1 != name2 {
                    node1
                        .sibling_refs
                        .insert(name2.clone(), FeShared::share(node2));
                }
            }
        }
    }

    parser::fill_project_node_scope(&mut root_node)?;
    analysis::analyze_and_fix(&mut root_node)?;

    // println!("\nAST: {root_node:#?}\n");

    return Ok(root_node);
}

pub fn compile_to_ferrum_mod_node(file: PathBuf, name: String) -> Result<FerrumModNode> {
    let path = file.to_path_buf();

    let content = fs::read_to_string(file)?;
    let tokens = lexer::lex_into_tokens(content)?;

    // println!("\nTokens: {tokens:#?}\n");

    let file_ast = parser::parse_to_ast(tokens)?;

    let node = FerrumModNode::new(
        name,
        path,
        FerrumModNodeFile::File(file_ast),
        parser::ast::FileType::Ferrum,
    );

    return Ok(node);
}

pub fn compile_to_rust_bindings_node(file: PathBuf, name: String) -> Result<FerrumModNode> {
    let path = file.to_path_buf();

    let content = fs::read_to_string(file)?;
    let tokens = lexer::lex_into_tokens(content)?;

    // println!("\nTokens: {tokens:#?}\n");

    let file_ast = parser::parse_rust_bindings_to_ast(tokens)?;

    let node = FerrumModNode::new(
        name,
        path,
        FerrumModNodeFile::File(file_ast),
        parser::ast::FileType::LocalRustBind,
    );

    return Ok(node);
}

pub fn translate_to_rust_ast(ferrum_ast: FeShared<FerrumModNode>) -> Result<RustProject> {
    let rs_ast = translator::translate_to_rust(ferrum_ast)?;

    // println!("\nRust AST: {rs_ast:#?}\n");

    return Ok(rs_ast);
}

pub fn generate_cargo_project(rust_ast: RustProject, build_dir: PathBuf) -> Result<CargoProject> {
    return Ok(generator::generate_cargo_project(rust_ast, build_dir)?);
}

pub fn cargo_build(cargo_project: CargoProject, target: Target, out_file: PathBuf) -> Result {
    cargo::build(cargo_project, target, out_file)?;

    return Ok(());
}
