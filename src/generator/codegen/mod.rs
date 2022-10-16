use super::*;

mod expr;
use expr::*;

mod fn_def;
use fn_def::*;

mod stmt;
use stmt::*;

mod r#type;
use r#type::*;

pub struct Generator {
    pub indent_count: usize,
}

impl Generator {
    pub fn new() -> Self {
        return Self { indent_count: 0 };
    }

    pub fn padding(&self) -> String {
        let mut left_pad = String::new();

        for _ in 0..self.indent_count {
            left_pad.push_str("    ");
        }

        return left_pad;
    }
}

pub fn generate_rust_code(project: RustProject) -> Result<GenProject> {
    fn rec_generate_rust_code(node: RustModNode) -> Result<GenNode> {
        match node.file {
            RustModNodeFile::File(file) => {
                let mut generator = Generator::new();
                let rs = gen_rs_for_file(&mut generator, file)?;

                return Ok(GenNode::File(GenFile {
                    name: node.name,
                    code: rs,
                }));
            }
            RustModNodeFile::Dir(nodes) => {
                let mut mods = vec![];

                for (_, node) in nodes {
                    let gen_mod = rec_generate_rust_code(node)?;
                    mods.push(gen_mod);
                }

                return Ok(GenNode::Dir(GenDir {
                    name: node.name,
                    files: mods,
                }));
            }
        }
    }

    let mut generator = Generator::new();
    let rs = gen_rs_for_file(&mut generator, project.main_file)?;

    let mut main_file = GenFile {
        name: String::from("main"),
        code: rs,
    };

    let mut siblings = vec![];

    for (_, sibling_ref) in project.siblings {
        let sibling = rec_generate_rust_code(sibling_ref)?;
        siblings.push(sibling);
    }

    return Ok(GenProject {
        main_file,
        siblings,
    });
}

pub fn gen_rs_for_file(generator: &mut Generator, file_ast: RustFileAst) -> Result<String> {
    let mut rs = String::new();

    rs.push_str("use crate::ferrum::prelude::*;\n\n");

    for item in file_ast.items {
        rs.push_str(&generator.padding());
        rs.push_str(&gen_rs_for_item(generator, item));
    }

    return Ok(rs);
}

fn gen_rs_for_item(generator: &mut Generator, item: Item) -> String {
    match item {
        Item::Use(use_node) => return gen_rs_for_use(generator, use_node),
        Item::Mod(mod_node) => return gen_rs_for_mod(generator, mod_node),
        Item::FnDef(fn_def) => return gen_rs_for_fn_def(generator, fn_def),
    }
}

fn gen_rs_for_mod(generator: &mut Generator, mod_node: Mod) -> String {
    let mut rs = String::new();

    if mod_node.is_public {
        rs.push_str("pub ");
    }

    rs.push_str("mod ");
    rs.push_str(&mod_node.name);
    rs.push_str(";\n");

    return rs;
}

fn gen_rs_for_use(generator: &mut Generator, use_node: Use) -> String {
    let mut rs = String::new();

    if use_node.is_public {
        rs.push_str("pub ");
    }

    rs.push_str("use ");
    rs.push_str(&gen_rs_for_use_pattern(use_node.use_pattern));
    rs.push_str(";\n");

    return rs;
}

fn gen_rs_for_use_pattern(use_pattern: UsePattern) -> String {
    match use_pattern {
        UsePattern::Id(id) => return id,
        UsePattern::Path(path) => {
            return format!("{}::{}", path.parent, gen_rs_for_use_pattern(*path.rhs))
        }
        UsePattern::Wild => return String::from("*"),
        UsePattern::Destruct(destruct) => {
            let mut rs = String::from("{");

            let fields: String = destruct
                .fields
                .into_iter()
                .map(gen_rs_for_use_pattern)
                .collect::<Vec<String>>()
                .join(", ");
            rs.push_str(&fields);

            rs.push('}');

            return rs;
        }
    }
}
