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

use parser::ast::{
    FerrumProjectAst,
    FerrumProjectAstNode,
    FerrumFileAst,
};

use translator::ast::RustProjectAst;

use cargo::project::CargoProject;

use std::{fs, path::PathBuf};

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
    let entry_ast = compile_to_ferrum_file_ast(entry_file)?;

    let mut project_ast = FerrumProjectAst {
        root: FerrumProjectAstNode {
            file: entry_ast,
            nodes: vec![],
        },
    };

    // TODO: recursively build ProjectAst from use locals

    parser::fill_project_scope_tables(&mut project_ast);

    println!("\nAST: {project_ast:#?}\n");

    return Ok(project_ast);
}

pub fn compile_to_ferrum_file_ast(file: PathBuf) -> Result<FerrumFileAst> {
    let content = fs::read_to_string(file)?;
    let tokens = lexer::lex_into_tokens(content)?;

    println!("\nTokens: {tokens:#?}\n");

    let file_ast = parser::parse_to_ast(tokens)?;

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
