use super::*;

use anyhow::ensure;

pub fn create_and_write_to_cargo_project(
    root_file: GenFile,
    build_dir: PathBuf,
) -> Result<CargoProject> {
    verify_cargo_installed()?;

    if let Err(_) = std::fs::canonicalize(&build_dir) {
        create_new_cargo_project(&build_dir)?;
    } else {
        let _ = std::fs::remove_dir_all(build_dir.join("src"));
        let _ = std::fs::create_dir(build_dir.join("src"));
    }

    let lib_file = build_dir.join("src/ferrum.rs");
    std::fs::write(lib_file, RUNTIME_RS)?;

    let name = String::from("main");

    write_gen_file(
        &build_dir,
        String::from("src"),
        name.clone(),
        root_file,
    )?;

    return Ok(CargoProject {
        build_dir,
        name,
    });
}

fn verify_cargo_installed() -> Result {
    let output = std::process::Command::new("cargo")
        .arg("--version")
        .output()?;

    ensure!(output.status.success(), "Error when checking cargo install");

    return Ok(());
}

fn create_new_cargo_project(build_dir: &PathBuf) -> Result {
    let output = std::process::Command::new("cargo")
        .args(&[
            "new",
            "--name",
            "main",
            "--vcs",
            "none",
            "--color",
            "never",
            "--quiet",
            &build_dir.to_string_lossy(),
        ])
        .output()?;

    if !output.status.success() {
        let stderr = output.stderr;
        let string = String::from_utf8(stderr)?;

        eprintln!("{}", string);

        ensure!(false, "Error when creating new cargo project");
    }

    return Ok(());
}

fn write_gen_file(build_dir: &PathBuf, pre: String, name: String, file: GenFile) -> Result {
    let filename = if file.mods.is_empty() || name.as_str() == "main" {
        format!("{name}.rs")
    } else {
        let _ = std::fs::create_dir(build_dir.join(&pre).join(&name));

        format!("{name}/mod.rs")
    };

    let path = build_dir.join(&pre).join(filename);

    std::fs::write(path, file.code)?;

    let pre = if name.as_str() == "main" {
        format!("{pre}")
    } else {
        format!("{pre}/{name}")
    };

    for (name, file) in file.mods.into_iter() {
        write_gen_file(build_dir, pre.clone(), name, file)?;
    }

    return Ok(());
}
