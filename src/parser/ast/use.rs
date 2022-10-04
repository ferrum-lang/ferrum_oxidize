use super::*;

#[derive(Debug, Clone)]
pub struct UseNode {
    pub public: Option<Token>,
    pub use_token: Token,
    pub use_pattern: UsePatternNode<InitUsePattern>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct UsePatternNode<T = UsePattern> {
    pub use_pattern: T,
    pub span: Span,
}

impl<T: Into<UsePattern>> UsePatternNode<T> {
    pub fn normalize(self) -> UsePatternNode<UsePattern> {
        return UsePatternNode {
            use_pattern: self.use_pattern.into(),
            span: self.span,
        };
    }
}

#[derive(Debug, Clone)]
pub enum InitUsePattern {
    Id(Token),
    Path(UsePatternPath),
}

#[derive(Debug, Clone)]
pub enum DestructInitUsePattern {
    Id(Token),
    Self_(Token),
    Path(UsePatternPath),
}

#[derive(Debug, Clone)]
pub enum UsePattern {
    Id(Token),
    Wild(Token),
    Path(UsePatternPath),
    Destruct(UsePatternDestruct),
}

impl From<InitUsePattern> for UsePattern {
    fn from(value: InitUsePattern) -> Self {
        match value {
            InitUsePattern::Id(id) => return UsePattern::Id(id),
            InitUsePattern::Path(p) => return UsePattern::Path(p),
        }
    }
}

impl From<DestructInitUsePattern> for UsePattern {
    fn from(value: DestructInitUsePattern) -> Self {
        match value {
            DestructInitUsePattern::Id(id) => return UsePattern::Id(id),
            DestructInitUsePattern::Self_(self_) => return UsePattern::Id(self_),
            DestructInitUsePattern::Path(p) => return UsePattern::Path(p),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UsePatternPath {
    pub parent_name: Token,
    pub delim: Token,
    pub rhs: Box<UsePatternNode>,
}

#[derive(Debug, Clone)]
pub struct UsePatternDestruct {
    pub open_brace: Token,
    pub patterns: Punctuated<Box<UsePatternNode<DestructInitUsePattern>>, Token>,
    pub close_brace: Token,
}
