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

use parser::ast::{FerrumFileAst, FerrumProjectAst, FerrumProjectAstNode};

use translator::ast::RustProjectAst;

use cargo::project::CargoProject;

use std::{fs, path::PathBuf};

use crate::parser::ast::ScopeTable;

#[derive(Debug, Clone)]
pub struct FerrumProject {
    pub entry_file: PathBuf,
    pub build_dir: PathBuf,
    pub out_file: PathBuf,
    pub target: Target,
}

pub fn build_project(cfg: Config) -> Result<FerrumProject> {
    let entry_file = config::determine_entry_file(cfg.entry_file)?;
    dbg!(&entry_file);

    let build_dir = config::determine_build_dir(cfg.build_dir)?;
    dbg!(&build_dir);

    let out_file = config::determine_out_file(cfg.out_file, &entry_file)?;
    dbg!(&out_file);

    let target = config::determine_target(cfg.target)?;
    dbg!(&target);

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

pub fn compile_to_ferrum_project_ast(entry_file: PathBuf) -> Result<FerrumProjectAst> {
    let name = if let Some(filename) = entry_file.file_stem() {
        filename.to_string_lossy().to_string()
    } else {
        todo!("Invalid file: {:?}", entry_file);
    };

    let entry_ast = compile_to_ferrum_file_ast(entry_file, name, true)?;

    let mut project_ast = FerrumProjectAst {
        root: FerrumProjectAstNode {
            file: entry_ast,
            nodes: vec![],
        },
    };

    fn rec_compile_file_into_node(
        is_dir: bool,
        filepath: PathBuf,
        name: String,
        is_mod_root: bool,
    ) -> Result<FerrumProjectAstNode> {
        if is_dir {
            let mut mod_file = None;
            let mut nodes = vec![];

            for file in filepath.read_dir()? {
                let file = file?;
                let filename = file.file_name().to_string_lossy().to_string();

                let filepath = filepath.join(&filename);

                if file.file_type()?.is_dir() {
                    let node = rec_compile_file_into_node(true, filepath, filename, false)?;
                    nodes.push(node);
                } else {
                    if !filename.ends_with(".fe") {
                        continue;
                    }

                    if filename == "_self.fe" {
                        let name = String::from("mod");
                        let node = rec_compile_file_into_node(false, filepath, name, true)?;
                        mod_file = Some(node.file);
                    } else {
                        let name = filename[..filename.len() - 3].to_string();

                        let node = rec_compile_file_into_node(false, filepath, name, false)?;
                        nodes.push(node);
                    }
                }
            }

            let file = mod_file.unwrap_or_else(|| FerrumFileAst {
                name,
                path: filepath.join("_self.fe"),
                is_mod_root: true,
                items: nodes
                    .iter()
                    .map(|node| parser::ast::ItemNode {
                        item: parser::ast::Item::Use(parser::ast::UseNode {
                            public: Some(lexer::token::Token {
                                literal: String::from("pub"),
                                token_type: lexer::token::TokenType::Keyword(
                                    lexer::token::TokenKeyword::Pub,
                                ),
                                span: span::Span::from((0, 0)),
                            }),
                            use_token: lexer::token::Token {
                                literal: String::from("use"),
                                token_type: lexer::token::TokenType::Keyword(
                                    lexer::token::TokenKeyword::Use,
                                ),
                                span: span::Span::from((0, 0)),
                            },
                            use_pattern: parser::ast::UsePatternNode {
                                use_pattern: parser::ast::InitUsePattern::Id(lexer::token::Token {
                                    literal: node.file.name.clone(),
                                    token_type: lexer::token::TokenType::Identifier,
                                    span: span::Span::from((0, 0)),
                                }),
                                span: span::Span::from((0, 0)),
                            },
                            span: span::Span::from((0, 0)),
                        }),
                        span: span::Span::from((0, 0)),
                    })
                    .collect(),
                scope: ScopeTable::new(),
                span: span::Span::from((0, 0)),
            });

            return Ok(FerrumProjectAstNode { file, nodes });
        }

        let file_ast = compile_to_ferrum_file_ast(filepath, name, is_mod_root)?;

        return Ok(FerrumProjectAstNode {
            file: file_ast,
            nodes: vec![],
        });
    }

    if let Some(dir) = project_ast.root.file.path.parent() {
        for file in dir.read_dir()? {
            let file = file?;
            let filename = file.file_name().to_string_lossy().to_string();

            if filename.starts_with(".") {
                continue;
            }

            let filepath = dir.join(&filename);

            if file.file_type()?.is_dir() {
                project_ast
                    .root
                    .nodes
                    .push(rec_compile_file_into_node(true, filepath, filename, false)?);
            } else {
                if !filename.ends_with(".fe") {
                    continue;
                }

                let name = filename[..filename.len() - 3].to_string();

                if name != project_ast.root.file.name {
                    project_ast
                        .root
                        .nodes
                        .push(rec_compile_file_into_node(false, filepath, name, false)?);
                }
            }
        }
    }

    parser::fill_project_scope_tables(&mut project_ast)?;

    println!("\nAST: {project_ast:#?}\n");

    return Ok(project_ast);
}

pub fn compile_to_ferrum_file_ast(
    file: PathBuf,
    name: String,
    is_mod_root: bool,
) -> Result<FerrumFileAst> {
    let path = file.to_path_buf();

    let content = fs::read_to_string(file)?;
    let tokens = lexer::lex_into_tokens(content)?;

    println!("\nTokens: {tokens:#?}\n");

    let file_ast = parser::parse_to_ast(name, path, is_mod_root, tokens)?;

    return Ok(file_ast);
}

pub fn translate_to_rust_ast(ferrum_ast: FerrumProjectAst) -> Result<RustProjectAst> {
    let rs_ast = translator::translate_to_rust(ferrum_ast)?;

    println!("\nRust AST: {rs_ast:#?}\n");

    return Ok(rs_ast);
}

pub fn generate_cargo_project(
    rust_ast: RustProjectAst,
    build_dir: PathBuf,
) -> Result<CargoProject> {
    return Ok(generator::generate_cargo_project(rust_ast, build_dir)?);
}

pub fn cargo_build(cargo_project: CargoProject, target: Target, out_file: PathBuf) -> Result {
    cargo::build(cargo_project, target, out_file)?;

    return Ok(());
}
