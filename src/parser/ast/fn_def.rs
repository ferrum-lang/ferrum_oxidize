use super::*;

#[derive(Debug, Clone)]
pub struct FnDefNode {
    pub name: Token,
    pub params: Vec<FnDefParamNode>,
    pub return_type: Option<TypeNode>,
    pub body: Vec<StatementNode>,
}

#[derive(Debug, Clone)]
pub struct FnDefParamNode {}

