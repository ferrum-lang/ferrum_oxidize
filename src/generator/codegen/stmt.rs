use super::*;

pub fn gen_rs_for_stmt(generator: &mut Generator, stmt: Statement) -> String {
    let mut rs = generator.padding();

    match stmt {
        Statement::Item(item) => {
            rs.push_str(&gen_rs_for_item(generator, item));

            // Semicolons not needed on items
            return rs;
        },
        _ => rs.push_str(&gen_rs_for_partial_stmt(generator, stmt)),
    }

    rs.push_str(";\n");

    return rs;
}

fn gen_rs_for_partial_stmt(generator: &mut Generator, stmt: Statement) -> String {
    let mut rs = String::new();

    match stmt {
        Statement::Item(_) => unreachable!(),
        Statement::Expr(expr) => {
            rs.push_str(&gen_rs_for_expr(generator, expr));
        },
        Statement::Decl(decl) => {
            if decl.is_const {
                rs.push_str("let ");
            } else {
                rs.push_str("let mut ");
            }

            rs.push_str(&gen_rs_for_decl_pattern(generator, decl.decl_pattern));

            if let Some(explicit_type) = decl.explicit_type {
                rs.push_str(": ");
                rs.push_str(&gen_rs_for_type(generator, explicit_type));
            }
        },
        Statement::Assign(assign) => {
            rs.push_str(&gen_rs_for_partial_stmt(generator, *assign.lhs));

            match assign.assign_type {
                AssignType::Eq => rs.push_str(" = "),
            }

            rs.push_str(&gen_rs_for_expr(generator, assign.rhs));
        },
    }

    return rs;
}

pub fn gen_rs_for_decl_pattern(_: &mut Generator, decl_pattern: DeclPattern) -> String {
    match decl_pattern {
        DeclPattern::Id(id) => return id,
    }
}

