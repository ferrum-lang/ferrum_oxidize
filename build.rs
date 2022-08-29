use std::env;
use std::fs;
use std::io;
use std::path::Path;

use rust_source_bundler::bundle_source;

use anyhow::Result;

const RUNTIME_ROOT_DIR: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/ferrum_runtime/src");

const GENERATED_DIR: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/generated");
const GENERATED_RUNTIME_FILE: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/generated/runtime.rs");

fn main() -> Result<()> {
    create_generated_dir()?;

    let code = bundle_source(RUNTIME_ROOT_DIR, String::from("lib.rs"))?;

    fs::write(
        &Path::new(GENERATED_RUNTIME_FILE),
        code,
    )?;
    
    return Ok(());
}

fn create_generated_dir() -> Result<(), io::Error> {
    match fs::read_dir(GENERATED_DIR) {
        Ok(_) => {},
        Err(_) => fs::create_dir(GENERATED_DIR)?,
    }

    return Ok(());
}

