use super::*;

#[derive(Debug, Clone)]
pub enum Statement {
    Item(Item),
    Expr(Expr),
    Decl(Declaration),
}

#[derive(Debug, Clone)]
pub struct Declaration {
    pub is_const: bool,
    pub assign_pattern: AssignPattern,
    pub explicit_type: Option<Type>,
    pub rhs: Option<Expr>,
}

#[derive(Debug, Clone)]
pub enum AssignPattern {
    Id(String),
}

