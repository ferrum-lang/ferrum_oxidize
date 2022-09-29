use super::*;

pub fn gen_rs_for_fn_def(generator: &mut Generator, fn_def: FnDef) -> String {
    let mut rs = String::new();

    if fn_def.is_public {
        rs.push_str("pub ");
    }

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

