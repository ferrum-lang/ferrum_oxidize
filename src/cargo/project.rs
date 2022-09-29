use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct CargoProject {
    pub build_dir: PathBuf,
    pub name: String,
}

