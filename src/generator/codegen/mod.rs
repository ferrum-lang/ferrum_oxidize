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
        return Self {
            indent_count: 0,
        };
    }

    pub fn padding(&self) -> String {
        let mut left_pad = String::new();

        for _ in 0..self.indent_count {
            left_pad.push_str("    ");
        }

        return left_pad;
    }
}

pub fn generate_rust_code(ast_node: RustProjectAstNode) -> Result<GenFile> {
    let mut generator = Generator::new();
    let rs = gen_rs_for_file(&mut generator, ast_node.file)?;

    let mut file = GenFile {
        code: rs,
        mods: HashMap::new(),
    };

    for (name, child) in ast_node.mods {
        let child_file = generate_rust_code(child)?;
        file.mods.insert(name, child_file);
    }

    return Ok(file);
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
        Item::FnDef(fn_def) => return gen_rs_for_fn_def(generator, fn_def),
    }
}

