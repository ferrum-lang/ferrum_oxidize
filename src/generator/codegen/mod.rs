use super::*;

pub fn gen_rs_for_file(generator: &mut Generator, file_ast: RustFileAst) -> Result<String> {
    let mut rs = String::new();

    rs.push_str("\nuse crate::ferrum::prelude::*;\n\n");

    for item in file_ast.items {
        rs.push_str(&gen_rs_for_item(generator, item));
    }

    return Ok(rs);
}

fn gen_rs_for_item(generator: &mut Generator, item: Item) -> String {
    match item {
        Item::FnDef(fn_def) => return gen_rs_for_fn_def(generator, fn_def),
    }
}

fn gen_rs_for_fn_def(generator: &mut Generator, fn_def: FnDef) -> String {
    let mut rs = generator.padding();
    rs.push_str(&format!("fn {}() {{\n", fn_def.name));

    generator.indent_count += 1;

    for stmt in fn_def.body {
        rs.push_str(&gen_rs_for_stmt(generator, stmt));
    }

    generator.indent_count -= 1;

    rs.push_str(&generator.padding());
    rs.push_str("}\n");

    return rs;
}

fn gen_rs_for_stmt(generator: &mut Generator, stmt: Statement) -> String {
    let mut rs = generator.padding();

    match stmt {
        Statement::Expr(expr) => {
            rs.push_str(&gen_rs_for_expr(generator, expr));
        },
        Statement::Item(item) => {
            rs.push_str(&gen_rs_for_item(generator, item));
        },
    }

    rs.push_str(";\n");

    return rs;
}

fn gen_rs_for_fn_call(generator: &mut Generator, fn_call: FnCall) -> String {
    let mut rs = format!("{}(", fn_call.name);

    for expr in fn_call.args {
        rs.push_str(&gen_rs_for_expr(generator, expr));
    }

    rs.push_str(")");

    return rs;
}

fn gen_rs_for_expr(generator: &mut Generator, expr: Expr) -> String {
    match expr {
        Expr::Literal(literal) => return gen_rs_for_lit(generator, literal),
        Expr::FnCall(fn_call) => return gen_rs_for_fn_call(generator, fn_call),
    }
}

fn gen_rs_for_lit(generator: &mut Generator, literal: Literal) -> String {
    match literal {
        Literal::Bool(is_true) => return gen_rs_for_lit_bool(generator, is_true),
        Literal::String(string) => return gen_rs_for_lit_string(generator, string),
    }
}

fn gen_rs_for_lit_bool(_: &mut Generator, is_true: bool) -> String {
    if is_true {
        return "true".to_string();
    } else {
        return "false".to_string();
    }
}

fn gen_rs_for_lit_string(_: &mut Generator, string: String) -> String {
    return format!("FeStr::from_static(\"{string}\")");
}

