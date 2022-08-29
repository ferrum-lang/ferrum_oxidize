mod result;

pub use result::Result;

const RUNTIME_RS: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/generated/runtime.rs"));

pub fn print_runtime() {
    println!("runtime code:\n{RUNTIME_RS}\n");
}

