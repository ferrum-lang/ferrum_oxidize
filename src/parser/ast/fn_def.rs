use super::*;

#[derive(Debug, Clone)]
pub struct FnDefNode {
    pub name: Token,
    pub params: Punctuated<FnDefParamNode, Token>,
    pub return_type: Option<TypeNode>,
    pub body: Vec<StatementNode>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct FnDefParamNode {
    pub name: Token,
    pub colon: Token,
    pub param_type: TypeNode,
}

