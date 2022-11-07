use super::*;

pub fn gen_rs_for_expr(generator: &mut Generator, expr: Expr) -> String {
    match expr {
        Expr::Literal(literal) => return gen_rs_for_lit(generator, literal),
        Expr::FnCall(fn_call) => return gen_rs_for_fn_call(generator, fn_call),
        Expr::StaticAccess(static_access) => return gen_rs_for_static_access(generator, static_access),
        Expr::IdentLookup(ident_lookup) => return ident_lookup,
        Expr::SharedRef(expr) => return format!("&{}", gen_rs_for_expr(generator, *expr)),
        Expr::MutRef(expr) => return format!("&mut {}", gen_rs_for_expr(generator, *expr)),
        Expr::Deref(expr) => return format!("*{}", gen_rs_for_expr(generator, *expr)),
    }
}

pub fn gen_rs_for_fn_call(generator: &mut Generator, fn_call: FnCall) -> String {
    let mut rs = format!("{}(", fn_call.name);

    let args = fn_call.args
        .into_iter()
        .map(|expr| gen_rs_for_expr(generator, expr))
        .collect::<Vec<String>>()
        .join(", ");

    rs.push_str(&args);

    rs.push_str(")");

    return rs;
}

pub fn gen_rs_for_lit(generator: &mut Generator, literal: Literal) -> String {
    match literal {
        Literal::Bool(is_true) => return gen_rs_for_lit_bool(generator, is_true),
        Literal::String(string) => return gen_rs_for_lit_string(generator, string),
        Literal::Tuple(exprs) => {
            let mut rs = String::from("(");

            for expr in exprs {
                rs.push_str(&gen_rs_for_expr(generator, expr));
                rs.push_str(",");
            }

            rs.push_str(")");

            return rs;
        },
        Literal::SomeOption(expr) => {
            let mut rs = String::from("Some(");

            rs.push_str(&gen_rs_for_expr(generator, *expr));

            rs.push_str(")");

            return rs;
        }
        Literal::NoneOption => return String::from("None"),
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

pub fn gen_rs_for_static_access(generator: &mut Generator, static_access: StaticAccess) -> String {
    return format!(
        "{}::{}",
        static_access.lhs,
        gen_rs_for_expr(generator, *static_access.rhs),
    );
}

