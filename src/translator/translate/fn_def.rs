use super::*;

pub fn translate_fn_def(translator: &mut Translator, fn_def: parser::ast::FnDefNode) -> Result<FnDef> {
    return Ok(FnDef {
        is_public: fn_def.pub_token.is_some(),
        name: fn_def.name.literal,
        params: fn_def
            .params
            .take_values()
            .into_iter()
            .map(|param| translate_fn_def_param(translator, param))
            .collect::<Result<Vec<FnDefParam>>>()?,
        return_type: if let Some((_, return_type)) = fn_def.return_type {
            Some(translate_type(translator, return_type)?)
        } else {
            None
        },
        body: fn_def
            .body
            .into_iter()
            .map(|item| match item.item {
                parser::ast::Item::Statement(stmt) => Ok(vec![translate_stmt(translator, stmt)?]),
                _ => Ok(translate_item(translator, item)?
                    .into_iter()
                    .map(|item| Statement::Item(item))
                    .collect::<Vec<Statement>>()),
            })
            .collect::<Result<Vec<Vec<Statement>>>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<Statement>>(),
    });
}

pub fn translate_fn_def_param(translator: &mut Translator, fn_def_param: parser::ast::FnDefParamNode) -> Result<FnDefParam> {
    return Ok(FnDefParam {
        name: fn_def_param.name.literal,
        param_type: translate_type(translator, fn_def_param.param_type)?,
    });
}
