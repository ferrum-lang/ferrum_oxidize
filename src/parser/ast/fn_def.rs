use super::*;

#[derive(Debug, Clone)]
pub struct FnDefNode {
    pub pub_token: Option<Token>,
    pub fn_token: Token,
    pub name: Token,
    pub generics: Option<GenericParamsNode>,
    pub open_paren: Token,
    pub params: Punctuated<FnDefParamNode, Token>,
    pub close_paren: Token,
    pub return_type: Option<(Token, TypeNode)>,
    pub body: FnDefBody,
    pub scope: ScopeTable,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct FnDefParamNode {
    pub name: Token,
    pub colon: Token,
    pub param_type: TypeNode,
    pub default_value: Option<(Token, ExprNode)>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum FnDefBody {
    Block(FnDefBlockNode),
    Stmt(FnDefStmtNode),
}

#[derive(Debug, Clone)]
pub struct FnDefBlockNode {
    pub items: Vec<FeShared<ItemNode>>,
    pub close_semicolon: Token,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct FnDefStmtNode {
    pub fat_arrow: Token,
    pub stmt: Box<FeShared<ItemNode>>,
    pub span: Span,
}

impl FnDefBody {
    pub fn get_items(&self) -> Vec<FeShared<ItemNode>> {
        match self {
            Self::Block(block) => {
                return block.items.iter().map(FeShared::share).collect();
            }
            Self::Stmt(stmt) => {
                return vec![FeShared::share(&*stmt.stmt)];
            }
        }
    }
}
