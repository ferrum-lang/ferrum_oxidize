use super::*;

pub fn translate_type(translator: &mut Translator, typ: parser::ast::TypeNode) -> Result<Type> {
    match typ.typ {
        parser::ast::Type::String(_) => return Ok(Type::FeStr),
        parser::ast::Type::SharedRef(shared_ref) =>
            return Ok(Type::SharedRef(Box::new(translate_type(translator, *shared_ref.of)?))),
        parser::ast::Type::MutRef(mut_ref) =>
            return Ok(Type::MutRef(Box::new(translate_type(translator, *mut_ref.of)?))),
        _ => todo!(),
    }
}
