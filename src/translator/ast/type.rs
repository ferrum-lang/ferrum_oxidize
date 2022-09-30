use super::*;

#[derive(Debug, Clone)]
pub enum Type {
    Isize,

    Usize,

    Char,

    FeStr,

    // Array(Box<Type>, ?),
    // Vec(Box<Type>),

    // Custom(CustomType),
    // DynInstance(CustomType),

    // Scoped(Box<TypeNode>, Box<TypeNode>),

    // SharedRef(Box<TypeNode>),
    // MutRef(Box<TypeNode>),

    // Managed(Box<TypeNode>),

    // Optional(Box<TypeNode>),
    // Result(Box<TypeNode>),
}

// #[derive(Debug, Clone)]
// pub struct CustomTypeNode {
//     pub name: Token,
//     pub generic_args: Option<GenericArgs>,
//     pub span: Span,
// }

// #[derive(Debug, Clone)]
// pub struct GenericArgs {
//     pub open_chevron: Token,
//     pub args: Punctuated<Box<TypeNode>, Token>,
//     pub close_chevron: Token,
//     pub span: Span,
// }

