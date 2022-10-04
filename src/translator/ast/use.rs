#[derive(Debug, Clone)]
pub struct Use {
    pub is_public: bool,
    pub use_pattern: UsePattern,
}

#[derive(Debug, Clone)]
pub enum UsePattern {
    Id(String),
    Path(UsePatternPath),
    Destruct(UsePatternDestruct),
    Wild,
}

#[derive(Debug, Clone)]
pub struct UsePatternPath {
    pub parent: String,
    pub rhs: Box<UsePattern>,
}

#[derive(Debug, Clone)]
pub struct UsePatternDestruct {
    pub fields: Vec<UsePattern>,
}

#[derive(Debug, Clone)]
pub struct Mod {
    pub is_public: bool,
    pub name: String,
}

