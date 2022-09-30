use super::*;

pub fn translate_type(typ: parser::ast::TypeNode) -> Result<Type> {
    match typ.typ {
        parser::ast::Type::String(_) => return Ok(Type::FeStr),
        _ => todo!(),
    }
}
