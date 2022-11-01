use super::*;

#[derive(Debug, Clone)]
pub struct DoNode {
    pub stmts: Vec<StatementNode>,
    pub close_semicolon: Token,
    pub span: Span,
}

