use super::*;

#[derive(Debug, Clone)]
pub struct GenericParamsNode {
    pub open_chevron: Token,
    pub params: Punctuated<GenericParamNode, Token>,
    pub close_chevron: Token,
}

#[derive(Debug, Clone)]
pub struct GenericParamNode {
    pub generic_param: GenericParam,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum GenericParam {}
