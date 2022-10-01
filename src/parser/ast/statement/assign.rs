use super::*;

#[derive(Debug, Clone)]
pub struct AssignPatternNode {
    pub assign_pattern: AssignPattern,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum AssignPattern {
    Id(Token),
    // ListDestruct(AssignPatternListDestruct),
}


