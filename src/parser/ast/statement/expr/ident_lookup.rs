use super::*;

#[derive(Debug, Clone)]
pub struct IdentLookupNode {
    pub name: Token,
    pub span: Span,
}
