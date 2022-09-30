use super::*;

#[derive(Debug, Clone)]
pub enum Statement {
    Item(Item),
    Expr(Expr),
    Assign(Assign),
}

#[derive(Debug, Clone)]
pub struct Assign {
    pub is_const: bool,
    pub name: String,
    pub explicit_type: Option<Type>,
    pub rhs: Option<Expr>,
}

