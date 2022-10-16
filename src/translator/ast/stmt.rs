use super::*;

#[derive(Debug, Clone)]
pub enum Statement {
    Item(Item),
    Expr(Expr),
    Decl(Declaration),
    Assign(Assign),
}

#[derive(Debug, Clone)]
pub struct Declaration {
    pub is_const: bool,
    pub decl_pattern: DeclPattern,
    pub explicit_type: Option<Type>,
}

#[derive(Debug, Clone)]
pub enum DeclPattern {
    Id(String),
}

#[derive(Debug, Clone)]
pub struct Assign {
    pub lhs: Box<Statement>,
    pub rhs: Expr,
    pub assign_type: AssignType,
}

#[derive(Debug, Clone)]
pub enum AssignType {
    Eq,
}

