use super::*;

#[derive(Debug, Clone)]
pub struct FnCallNode {
    pub name: Token,
    pub open_paren: Token,
    pub close_paren: Token,
    pub args: Punctuated<FnCallArgNode, Token>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct FnCallArgNode {
    pub name: Option<(Token, Token)>,
    pub expr: Box<ExprNode>,
    pub span: Span,
}


