use super::*;

pub fn translate_type(typ: parser::ast::TypeNode) -> Result<Type> {
    match typ.typ {
        parser::ast::Type::String(_) => return Ok(Type::FeStr),
        parser::ast::Type::SharedRef(shared_ref) =>
            return Ok(Type::SharedRef(Box::new(translate_type(*shared_ref.of)?))),
        parser::ast::Type::MutRef(mut_ref) =>
            return Ok(Type::MutRef(Box::new(translate_type(*mut_ref.of)?))),
        _ => todo!(),
    }
}
