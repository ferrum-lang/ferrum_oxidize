#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String,
    Char,
    Number,
    Bool(bool),
    Result { is_ok: bool },
    Option { is_some: bool },
}

