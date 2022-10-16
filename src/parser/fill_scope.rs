use super::*;

use std::collections::HashSet;

pub fn fill_mod_node_scope(ast_node: &mut FeShared<FerrumModNode>) -> Result {
    let mut visited = HashSet::new();
    fill_mod_node_pub_api(ast_node, &mut visited, true)?;

    let mut visited = HashSet::new();
    fill_mod_node_full_scope(ast_node, &mut visited, true)?;

    return Ok(());
}

fn fill_mod_node_pub_api(
    ast_node: &mut FeShared<FerrumModNode>,
    visited: &mut HashSet<usize>,
    is_entry: bool,
) -> Result {
    if visited.contains(&ast_node.id) {
        return Ok(());
    }

    match &mut FeShared::share(ast_node).file {
        FerrumModNodeFile::File(file) => {
            if file.pub_api.is_empty() {
                let mut pub_api = file.pub_api.clone();
                let mut items = file.items.clone();

                fill_scope_with_items(ast_node, &mut pub_api, &mut items, true, is_entry)?;

                file.pub_api = pub_api;
                file.items = items;
            }
        }
        FerrumModNodeFile::Dir(nodes) => {
            for (_, node) in nodes.iter_mut() {
                fill_mod_node_pub_api(node, visited, false)?;
            }
        }
    }

    visited.insert(ast_node.id);

    for (_, sibling_ref) in ast_node.sibling_refs.iter_mut() {
        fill_mod_node_pub_api(sibling_ref, visited, false)?;
    }

    return Ok(());
}

fn fill_mod_node_full_scope(
    ast_node: &mut FeShared<FerrumModNode>,
    visited: &mut HashSet<usize>,
    is_entry: bool,
) -> Result {
    if visited.contains(&ast_node.id) {
        return Ok(());
    }

    match &mut FeShared::share(ast_node).file {
        FerrumModNodeFile::File(file) => {
            if file.scope.is_empty() {
                let mut scope = file.scope.clone();
                let mut items = file.items.clone();

                fill_scope_with_items(ast_node, &mut scope, &mut items, false, is_entry)?;

                file.scope = scope;
                file.items = items;
            }
        }
        FerrumModNodeFile::Dir(nodes) => {
            for (_, node) in nodes.iter_mut() {
                fill_mod_node_full_scope(node, visited, false)?;
            }
        }
    }

    visited.insert(ast_node.id);

    for (_, sibling_ref) in ast_node.sibling_refs.iter_mut() {
        fill_mod_node_full_scope(sibling_ref, visited, false)?;
    }

    return Ok(());
}

fn fill_scope_with_items<'a>(
    ast_node: &mut FeShared<FerrumModNode>,
    scope: &mut ScopeTable,
    items: impl IntoIterator<Item = &'a mut FeShared<ItemNode>>,
    filling_pub_api: bool,
    is_entry: bool,
) -> Result {
    let is_pkg = ast_node.name.as_str() == "_pkg";

    for item in items {
        match &mut item.item {
            Item::Use(use_node) => {
                let is_public = use_node.public.is_some();

                if is_public && is_entry {
                    todo!("Error: Entry files can only export a main function.");
                }

                if is_public && !is_pkg {
                    todo!("Error: Cannot `pub use ...` from non-pkg files. This will only be allowed once external dependencies are supported.");
                }

                if is_public && use_node.pattern_prefix.is_some() {
                    todo!("Error: Cannot `pub use ../...` or `pub use ~/...`");
                }

                if filling_pub_api && !is_public {
                    continue;
                }

                let dependency_name = match &use_node.use_pattern.use_pattern {
                    InitUsePattern::Id(id) => id.literal.clone(),
                    InitUsePattern::Path(path) => path.parent_name.literal.clone(),
                };

                if dependency_name == ast_node.name {
                    Err(ParseError::CannotUseSelf(
                        file!(),
                        line!(),
                        ast_node.path.clone(),
                        use_node.clone(),
                    ))?
                }

                let mut dependency_node = match &use_node.pattern_prefix {
                    Some(PatternPrefix::Root(_)) => {
                        let mut root_node = FeShared::share(ast_node);

                        while let Some(parent) = root_node.parent_ref.as_mut() {
                            root_node = FeShared::share(parent);
                        }

                        if root_node.name.as_str() == dependency_name {
                            root_node
                        } else {
                            if let Some(dep) = root_node.sibling_refs.get(&dependency_name) {
                                FeShared::share(dep)
                            } else {
                                todo!("Error! Can't find dependency");
                            }
                        }
                    }
                    Some(PatternPrefix::Rel(rel)) => {
                        let mut root_node = FeShared::share(ast_node);

                        for _ in rel.parent_dirs.iter() {
                            if let Some(parent) = ast_node.parent_ref.as_mut() {
                                root_node = FeShared::share(parent);
                            } else {
                                todo!("Error! Can't find super");
                            }
                        }

                        if root_node.name.as_str() == dependency_name {
                            root_node
                        } else {
                            if let Some(dep) = root_node.sibling_refs.get(&dependency_name) {
                                FeShared::share(dep)
                            } else {
                                todo!("Error! Can't find dependency");
                            }
                        }
                    }
                    None => {
                        if let Some(dep) = ast_node.sibling_refs.get(&dependency_name) {
                            FeShared::share(dep)
                        } else {
                            todo!("Error! Can't find dependency");
                        }
                    }
                };

                let mut tmp_dep_node = FeShared::share(&dependency_node);
                let dependency_file = match &mut tmp_dep_node.file {
                    FerrumModNodeFile::File(file) => file,
                    FerrumModNodeFile::Dir(files) => {
                        if let Some(pkg) = files.get_mut("_pkg") {
                            match &mut pkg.file {
                                FerrumModNodeFile::File(file) => file,
                                _ => todo!("Uh oh! _pkg must be a file."),
                            }
                        } else {
                            todo!("Uh oh! default _pkg wasn't generated properlly.")
                        }
                    }
                };

                // let dependency_node = if let Some(dep_node) = ast_node
                //     .nodes
                //     .iter_mut()
                //     .find(|n| n.file.name == dependency)
                // {
                //     dep_node
                // } else {
                //     Err(ParseError::UseNotFound(
                //         file!(),
                //         line!(),
                //         ast_node.file.path.clone(),
                //         use_node.clone(),
                //     ))?
                // };

                if filling_pub_api && dependency_file.pub_api.is_empty() {
                    let mut pub_api = dependency_file.pub_api.clone();
                    let mut items = dependency_file.items.clone();

                    fill_scope_with_items(
                        &mut dependency_node,
                        &mut pub_api,
                        &mut items,
                        filling_pub_api,
                        false,
                    )?;

                    dependency_file.pub_api = pub_api;
                    dependency_file.items = items;
                }

                match &mut use_node.use_pattern.use_pattern {
                    InitUsePattern::Id(id) => {
                        scope.insert(
                            id.literal.clone(),
                            ScopeRefNode {
                                is_public,
                                name: dependency_node.name.clone(),
                                scope_ref: ScopeRef::Mod(dependency_file.pub_api.clone()),
                            },
                        );
                    }
                    InitUsePattern::Path(path) => {
                        if let UsePattern::Destruct(UsePatternDestruct {
                            patterns,
                            open_brace,
                            close_brace,
                        }) = &path.rhs.use_pattern
                        {
                            let mut new_patterns = Punctuated::new();
                            let mut prev_delim = None;

                            for (pattern, delim) in patterns.clone().take_as_vec() {
                                if let DestructInitUsePattern::Self_(_) = pattern.use_pattern {
                                    scope.insert(
                                        path.parent_name.literal.clone(),
                                        ScopeRefNode {
                                            is_public,
                                            name: dependency_node.name.clone(),
                                            scope_ref: ScopeRef::Mod(
                                                dependency_file.pub_api.clone(),
                                            ),
                                        },
                                    );
                                } else {
                                    new_patterns.push(prev_delim, pattern);
                                    prev_delim = delim;
                                }
                            }

                            path.rhs.use_pattern = UsePattern::Destruct(UsePatternDestruct {
                                open_brace: open_brace.clone(),
                                patterns: new_patterns,
                                close_brace: close_brace.clone(),
                            });
                        }

                        resolve_use_pattern(scope, &dependency_file.pub_api, &path.rhs, is_public)?
                    }
                }
            }
            Item::FnDef(fn_def) => {
                if filling_pub_api && fn_def.pub_token.is_none() {
                    continue;
                }

                let mut is_main = false;

                if is_entry && fn_def.name.literal.as_str() == "main" {
                    if fn_def.pub_token.is_none() {
                        todo!("Error: Main fn must be public.");
                    }
                    
                    if fn_def.generics.is_some() {
                        todo!("Error: Main fn cannot contain generic params.");
                    }

                    if let Some((_, return_type)) = &fn_def.return_type {
                        match return_type.typ {
                            Type::Result(None) => {},
                            _ => todo!("Error: Invalid return type for main fn."),
                        }
                    }
                    
                    is_main = true;
                }

                if is_entry && fn_def.pub_token.is_some() && !is_main {
                    todo!("Error: Entry files can only export a main function.");
                }

                scope.insert(
                    fn_def.name.literal.clone(),
                    ScopeRefNode {
                        is_public: fn_def.pub_token.is_some(),
                        name: fn_def.name.literal.clone(),
                        scope_ref: ScopeRef::Fn {
                            name: fn_def.name.literal.clone(),
                            generics: fn_def.generics.clone().map(|g| {
                                g.params
                                    .take_values()
                                    .into_iter()
                                    .map(|g| g.generic_param)
                                    .collect::<Vec<GenericParam>>()
                            }),
                            params: fn_def.params.clone().take_values(),
                            return_type: fn_def.return_type.clone().map(|r| r.1.typ),
                        },
                    },
                );
                fill_scope_with_items(
                    ast_node,
                    &mut fn_def.scope,
                    &mut fn_def.body,
                    filling_pub_api,
                    false,
                )?;
            }
            Item::Statement(_) => {}
        }
    }

    return Ok(());
}

fn resolve_use_pattern(
    dest_scope: &mut ScopeTable,
    src_scope: &ScopeTable,
    use_pattern: &UsePatternNode,
    is_public: bool,
) -> Result {
    match &use_pattern.use_pattern {
        UsePattern::Id(id) => {
            let scope_ref = if let Some(scope_ref) = src_scope.get(&id.literal) {
                scope_ref
            } else {
                Err(ParseError::InvalidUsePattern(
                    file!(),
                    line!(),
                    use_pattern.clone(),
                ))?
            };

            if !scope_ref.is_public {
                Err(ParseError::CannotUsePrivate(
                    file!(),
                    line!(),
                    use_pattern.clone(),
                    scope_ref.clone(),
                ))?;
            }

            dest_scope.insert(
                id.literal.clone(),
                ScopeRefNode {
                    name: id.literal.clone(),
                    is_public,
                    scope_ref: public_only_scope_ref(scope_ref.scope_ref.clone()),
                },
            );
        }
        UsePattern::Path(path) => {
            let scope_ref = if let Some(scope_ref) = src_scope.get(&path.parent_name.literal) {
                scope_ref
            } else {
                Err(ParseError::InvalidUsePattern(
                    file!(),
                    line!(),
                    use_pattern.clone(),
                ))?
            };

            if !scope_ref.is_public {
                Err(ParseError::CannotUsePrivate(
                    file!(),
                    line!(),
                    use_pattern.clone(),
                    scope_ref.clone(),
                ))?;
            }

            let mut path = path.clone();

            path.rhs.use_pattern = if let UsePattern::Destruct(UsePatternDestruct {
                patterns,
                open_brace,
                close_brace,
            }) = path.rhs.use_pattern.clone()
            {
                let mut new_patterns = Punctuated::new();

                for (pattern, delim) in patterns.clone().take_as_vec() {
                    if let DestructInitUsePattern::Self_(_) = pattern.use_pattern {
                        dest_scope.insert(
                            path.parent_name.literal.clone(),
                            ScopeRefNode {
                                name: path.parent_name.literal.clone(),
                                is_public,
                                scope_ref: public_only_scope_ref(scope_ref.scope_ref.clone()),
                            },
                        );
                    } else {
                        new_patterns.push(delim, pattern);
                    }
                }

                UsePattern::Destruct(UsePatternDestruct {
                    open_brace,
                    patterns: new_patterns,
                    close_brace,
                })
            } else {
                path.rhs.use_pattern
            };

            match &scope_ref.scope_ref {
                ScopeRef::Mod(mod_scope) => {
                    resolve_use_pattern(dest_scope, &mod_scope, &path.rhs, is_public)?
                }
                ScopeRef::Fn { .. } => Err(ParseError::InvalidUsePattern(
                    file!(),
                    line!(),
                    use_pattern.clone(),
                ))?,
            }
        }
        UsePattern::Wild(_) => {
            for (name, scope_ref) in src_scope {
                if scope_ref.is_public {
                    dest_scope.insert(
                        name.clone(),
                        ScopeRefNode {
                            name: name.clone(),
                            is_public,
                            scope_ref: public_only_scope_ref(scope_ref.scope_ref.clone()),
                        },
                    );
                }
            }
        }
        UsePattern::Destruct(destruct) => {
            for use_pattern in destruct.patterns.clone().take_values() {
                resolve_use_pattern(dest_scope, src_scope, &use_pattern.normalize(), is_public)?;
            }
        }
    }

    return Ok(());
}

fn public_only_scope_ref(scope_ref: ScopeRef) -> ScopeRef {
    match scope_ref {
        ScopeRef::Mod(scope) => {
            return ScopeRef::Mod(
                scope
                    .into_iter()
                    .filter_map(|(k, v)| if v.is_public { Some((k, v)) } else { None })
                    .collect::<ScopeTable>(),
            )
        }
        _ => return scope_ref,
    }
}
