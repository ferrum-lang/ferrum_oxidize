use super::*;

pub fn translate_fn_def(fn_def: parser::ast::FnDefNode) -> Result<FnDef> {
    return Ok(FnDef {
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
            .map(translate_stmt)
            .collect::<Result<Vec<Statement>>>()?,
    });
}

pub fn translate_fn_def_param(fn_def_param: parser::ast::FnDefParamNode) -> Result<FnDefParam> {
    todo!();
}


