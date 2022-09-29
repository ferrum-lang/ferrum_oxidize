use super::*;

pub fn gen_rs_for_expr(generator: &mut Generator, expr: Expr) -> String {
    match expr {
        Expr::Literal(literal) => return gen_rs_for_lit(generator, literal),
        Expr::FnCall(fn_call) => return gen_rs_for_fn_call(generator, fn_call),
    }
}

pub fn gen_rs_for_fn_call(generator: &mut Generator, fn_call: FnCall) -> String {
    let mut rs = format!("{}(", fn_call.name);

    for expr in fn_call.args {
        rs.push_str(&gen_rs_for_expr(generator, expr));
    }

    rs.push_str(")");

    return rs;
}

pub fn gen_rs_for_lit(generator: &mut Generator, literal: Literal) -> String {
    match literal {
        Literal::Bool(is_true) => return gen_rs_for_lit_bool(generator, is_true),
        Literal::String(string) => return gen_rs_for_lit_string(generator, string),
    }
}

pub fn gen_rs_for_lit_bool(_: &mut Generator, is_true: bool) -> String {
    if is_true {
        return "true".to_string();
    } else {
        return "false".to_string();
    }
}

pub fn gen_rs_for_lit_string(_: &mut Generator, string: String) -> String {
    return format!("FeStr::from_static(\"{string}\")");
}
