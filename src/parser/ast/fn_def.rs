use super::*;

#[derive(Debug, Clone)]
pub struct FnDefNode {
    pub pub_token: Option<Token>,
    pub fn_token: Token,
    pub name: Token,
    pub open_paren: Token,
    pub params: Punctuated<FnDefParamNode, Token>,
    pub close_paren: Token,
    pub return_type: Option<(Token, TypeNode)>,
    pub open_brace: Token,
    pub body: Vec<ItemNode>,
    pub close_brace: Token,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct FnDefParamNode {
    pub name: Token,
    pub colon: Token,
    pub param_type: TypeNode,
    pub default_value: Option<(Token, ExprNode)>,
    pub span: Span,
}

