mod error;
pub mod project;

pub use error::CargoError;

use crate::Target;
use crate::Result;

use std::path::PathBuf;

use anyhow::ensure;

pub fn build(cargo_project: project::CargoProject, target: Target, out_file: PathBuf) -> Result {
    let output = std::process::Command::new("cargo")
        .arg("build")
        .current_dir(cargo_project.build_dir.clone())
        .output()?;

    if !output.status.success() {
        let stderr = output.stderr;
        let string = String::from_utf8(stderr)?;

        eprintln!("{}", string);

        ensure!(false, "Error when building cargo project");
    }

    std::fs::rename(
        cargo_project.build_dir.join("target").join("debug").join("main"),
        out_file,
    )?;

    return Ok(());
}

// cargo +nightly rustc --profile=check -- -Zunpretty=expanded
pub fn expand(file: PathBuf, name: String) -> Result<String> {
    todo!();
}

