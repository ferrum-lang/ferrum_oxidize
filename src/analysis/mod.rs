use super::*;

pub fn analyze_and_fix(root_mod_node: &mut FeShared<FerrumModNode>) -> Result {
    // TODO
    // - Type resolution
    // - Const resolution
    // - Fn/Struct arg resolution
    // - Control flow resolution (ie. `do return it if cache[n]`)
    // - Number bounds & validation resolution (bounds on numbers for fn params; whether math returns result, etc)

    return Ok(());
}
