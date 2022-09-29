use super::*;

pub fn gen_rs_for_stmt(generator: &mut Generator, stmt: Statement) -> String {
    let mut rs = generator.padding();

    match stmt {
        Statement::Expr(expr) => {
            rs.push_str(&gen_rs_for_expr(generator, expr));
        },
        Statement::Item(item) => {
            rs.push_str(&gen_rs_for_item(generator, item));

            // Semicolons not needed on items
            return rs;
        },
    }

    rs.push_str(";\n");

    return rs;
}

