use super::*;

pub fn translate_type(typ: parser::ast::TypeNode) -> Result<Type> {
    match typ.typ {
        parser::ast::Type::String(_) => return Ok(Type::FeStr),
        parser::ast::Type::SharedRef(inner) =>
            return Ok(Type::SharedRef(Box::new(translate_type(*inner)?))),
        parser::ast::Type::MutRef(inner) =>
            return Ok(Type::MutRef(Box::new(translate_type(*inner)?))),
        _ => todo!(),
    }
}
