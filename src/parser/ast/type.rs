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

    SharedRef(Box<TypeNode>),
    MutRef(Box<TypeNode>),

    Managed(Box<TypeNode>),

    Optional(Box<TypeNode>),
    Result(Box<TypeNode>),
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

