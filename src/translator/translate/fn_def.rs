use super::*;

pub fn translate_fn_def(fn_def: parser::ast::FnDefNode) -> Result<FnDef> {
    return Ok(FnDef {
        is_public: fn_def.pub_token.is_some(),
        name: fn_def.name.literal,
        params: fn_def
            .params
            .take_values()
            .into_iter()
            .map(translate_fn_def_param)
            .collect::<Result<Vec<FnDefParam>>>()?,
        return_type: None,
        body: fn_def
            .body
            .into_iter()
            .map(|item| {
                match item.item {
                    parser::ast::Item::Statement(stmt) => translate_stmt(stmt),
                    _ => Ok(Statement::Item(translate_item(item)?)),
                }
            })
            .collect::<Result<Vec<Statement>>>()?,
    });
}

pub fn translate_fn_def_param(fn_def_param: parser::ast::FnDefParamNode) -> Result<FnDefParam> {
    todo!();
}


