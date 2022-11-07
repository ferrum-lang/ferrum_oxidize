#[derive(Debug, Clone)]
pub enum Type {
    Isize,

    Usize,

    Char,

    FeStr,

    Bool,

    // Array(Box<Type>, ?),
    // Vec(Box<Type>),

    // Custom(CustomType),
    // DynInstance(CustomType),

    // Scoped(Box<Type>, Box<Type>),

    SharedRef(Box<Type>),
    MutRef(Box<Type>),

    // Managed(Box<Type>),

    Optional(Box<Type>),
    // Result(Box<Type>),
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

