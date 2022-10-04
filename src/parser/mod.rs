pub mod ast;
use ast::*;

mod error;
pub use error::ParseError;

mod parse;
pub use parse::*;

use crate::lexer::token::*;
use crate::Result;

use std::path::PathBuf;

use anyhow::Context;

pub fn parse_to_ast(
    name: String,
    path: PathBuf,
    is_mod_root: bool,
    tokens: Vec<Token>,
) -> Result<FerrumFileAst> {
    let mut parser = Parser::new(tokens);
    return parse_file(name, path, is_mod_root, &mut parser);
}

pub fn fill_project_scope_tables(project_ast: &mut FerrumProjectAst) -> Result {
    fn fill_scope_with_items<'a>(
        ast_node: &mut FerrumProjectAstNode,
        scope: &mut ScopeTable,
        items: impl IntoIterator<Item = &'a mut ItemNode>,
    ) -> Result {
        for item in items {
            match &mut item.item {
                Item::Use(use_node) => {
                    let dependency = match &use_node.use_pattern.use_pattern {
                        InitUsePattern::Id(id) => id.literal.clone(),
                        InitUsePattern::Path(path) => path.parent_name.literal.clone(),
                    };

                    if dependency == ast_node.file.name {
                        Err(ParseError::CannotUseSelf(
                            file!(),
                            line!(),
                            ast_node.file.path.clone(),
                            use_node.clone(),
                        ))?
                    }

                    let dependency_node = if let Some(dep_node) = ast_node
                        .nodes
                        .iter_mut()
                        .find(|n| n.file.name == dependency)
                    {
                        dep_node
                    } else {
                        Err(ParseError::UseNotFound(
                            file!(),
                            line!(),
                            ast_node.file.path.clone(),
                            use_node.clone(),
                        ))?
                    };

                    if dependency_node.file.scope.is_empty() {
                        let mut scope = dependency_node.file.scope.clone();
                        let mut items = dependency_node.file.items.clone();

                        fill_scope_with_items(dependency_node, &mut scope, &mut items)?;

                        dependency_node.file.scope = scope;
                        dependency_node.file.items = items;
                    }

                    let is_public = use_node.public.is_some();

                    match &mut use_node.use_pattern.use_pattern {
                        InitUsePattern::Id(id) => {
                            scope.insert(
                                id.literal.clone(),
                                ScopeRefNode {
                                    is_public,
                                    name: dependency_node.file.name.clone(),
                                    scope_ref: public_only_scope_ref(ScopeRef::Mod(
                                        dependency_node.file.scope.clone(),
                                    )),
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
                                                name: dependency_node.file.name.clone(),
                                                scope_ref: public_only_scope_ref(ScopeRef::Mod(
                                                    dependency_node.file.scope.clone(),
                                                )),
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

                            resolve_use_pattern(
                                scope,
                                &dependency_node.file.scope,
                                &path.rhs,
                                is_public,
                            )?
                        }
                    }
                }
                Item::FnDef(fn_def) => {
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
                    fill_scope_with_items(ast_node, &mut fn_def.scope, &mut fn_def.body)?;
                }
                Item::Statement(_) => {}
            }
        }

        return Ok(());
    }

    fn handle_ast_node(ast_node: &mut FerrumProjectAstNode) -> Result {
        let mut scope = ast_node.file.scope.clone();
        let mut items = ast_node.file.items.clone();

        fill_scope_with_items(ast_node, &mut scope, &mut items)?;

        ast_node.file.scope = scope;
        ast_node.file.items = items;

        for node in &mut ast_node.nodes {
            handle_ast_node(node)?;
        }

        return Ok(());
    }

    handle_ast_node(&mut project_ast.root)?;

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

pub struct Parser {
    pub tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        return Self { tokens, index: 0 };
    }

    fn current(&self) -> Result<Token> {
        if self.index >= self.tokens.len() {
            Err(ParseError::IndexOutOfBounds(file!(), line!()))?;
        }

        return Ok(self.tokens[self.index].clone());
    }

    fn next(&self) -> Result<Token> {
        if self.index + 1 >= self.tokens.len() {
            Err(ParseError::IndexOutOfBounds(file!(), line!()))?;
        }

        return Ok(self.tokens[self.index + 1].clone());
    }

    fn scan(&self, token_types: &[TokenType]) -> bool {
        let mut index: usize = self.index;

        for i in 0..token_types.len() {
            if index >= self.tokens.len() {
                return false;
            }

            if self.tokens[index].token_type != token_types[i] {
                return false;
            }

            index += 1;
        }

        return true;
    }

    fn expect(&self, token_type: TokenType) -> Result<Token> {
        match self.current() {
            Ok(token) if token.token_type == token_type => return Ok(token),
            Ok(token) => Err(ParseError::NotExpectedToken(
                file!(),
                line!(),
                Some(token),
                token_type,
            ))?,
            Err(e) => Err(e).with_context(|| {
                ParseError::NotExpectedToken(file!(), line!(), None, token_type)
            })?,
        }
    }

    fn consume(&mut self, token_type: TokenType) -> Result<Token> {
        let token = self.expect(token_type)?;

        self.index += 1;

        return Ok(token);
    }

    fn consume_if(&mut self, token_type: TokenType) -> Result<Option<Token>> {
        if !self.scan(&[token_type.clone()]) {
            return Ok(None);
        }

        let token = self.consume(token_type)?;
        return Ok(Some(token));
    }

    fn consume_current(&mut self) -> Result<Token> {
        let token = self.current()?;

        self.index += 1;

        return Ok(token);
    }
}
