use super::*;

pub fn gen_rs_for_fn_def(generator: &mut Generator, fn_def: FnDef) -> String {
    let mut rs = String::new();

    if fn_def.is_public {
        rs.push_str("pub ");
    }

    rs.push_str(&format!("fn {}(", fn_def.name));

    for param in fn_def.params {
        rs.push_str(&gen_rs_for_fn_def_param(generator, param));
        rs.push_str(", ");
    }

    rs.push_str(")");

    if let Some(return_type) = fn_def.return_type {
        rs.push_str(" -> ");
        rs.push_str(&gen_rs_for_type(generator, return_type))
    }

    rs.push_str(" {\n");

    generator.indent_count += 1;

    for stmt in fn_def.body {
        rs.push_str(&gen_rs_for_stmt(generator, stmt));
    }

    generator.indent_count -= 1;

    rs.push_str(&generator.padding());
    rs.push_str("}\n");

    return rs;
}

pub fn gen_rs_for_fn_def_param(generator: &mut Generator, param: FnDefParam) -> String {
    let mut rs = String::new();

    match param.param_type {
        Type::SharedRef(_) | Type::MutRef(_) => {},
        _ => {
            rs.push_str("mut ");
        },
    }

    rs.push_str(&format!("{}: ", param.name));

    rs.push_str(&gen_rs_for_type(generator, param.param_type));

    return rs;
}

