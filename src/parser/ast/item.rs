use super::*;

#[derive(Debug, Clone)]
pub struct ItemNode {
    pub item: Item,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Item {
    Statement(StatementNode),
    FnDef(FnDefNode),
}

