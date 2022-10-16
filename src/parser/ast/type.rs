use super::*;

#[derive(Debug, Clone)]
pub struct TypeNode {
    pub typ: Type,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Type {
    Int(Token),

    Uint(Token),

    Char(Token),

    String(Token),

    Array(Box<TypeNode>, Token),
    List(Box<TypeNode>),

    Custom(CustomTypeNode),
    DynInstance(CustomTypeNode),

    Scoped(Box<TypeNode>, Box<TypeNode>),

    SharedRef(SharedRefNode),
    MutRef(MutRefNode),

    Managed(Box<TypeNode>),

    Optional(Box<TypeNode>),
    Result(Option<Box<TypeNode>>),
}

#[derive(Debug, Clone)]
pub struct CustomTypeNode {
    pub name: Token,
    pub generic_args: Option<GenericArgs>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct GenericArgs {
    pub open_chevron: Token,
    pub args: Punctuated<Box<TypeNode>, Token>,
    pub close_chevron: Token,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct SharedRefNode {
    pub ref_token: Token,
    pub of: Box<TypeNode>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct MutRefNode {
    pub ref_token: Token,
    pub mut_token: Token,
    pub of: Box<TypeNode>,
    pub span: Span,
}

