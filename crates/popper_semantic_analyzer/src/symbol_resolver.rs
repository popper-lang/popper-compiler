use crate::error::{Result, SemanticError};
use crate::{LayerOutput, SemanticAnalyzer, SemanticLayer};
use popper_ast::ast::{Expr, LangAst, LangNodeId, LangNodeKind, Span, SymbolId};
use popper_ast::layer::Layer;
use popper_ast::type_::Type;

#[derive(Debug, Clone)]
pub struct SymbolStorage {
    pub id: SymbolId,
    pub ty: Type,
    pub span: Span,
    pub used_count: usize,
}

#[derive(Debug, Clone)]
pub struct Scope {
    parent: Option<Box<Scope>>,
    symbols: Vec<SymbolStorage>,
}

impl Default for Scope {
    fn default() -> Self {
        Scope {
            parent: None,
            symbols: Vec::new(),
        }
    }
}

impl Scope {
    pub fn new_root() -> Self {
        Scope {
            parent: None,
            symbols: Vec::new(),
        }
    }

    pub fn new(parent: Scope) -> Self {
        Scope {
            parent: Some(Box::new(parent)),
            symbols: Vec::new(),
        }
    }

    pub fn create_child(&self) -> Self {
        Scope::new(self.clone())
    }

    pub fn insert(&mut self, id: SymbolId, ty: Type, span: Span) {
        self.symbols.push(SymbolStorage {
            id,
            ty,
            span,
            used_count: 0,
        });
    }

    pub fn get(&self, id: SymbolId) -> Option<&Type> {
        for symbol in &self.symbols {
            if symbol.id == id {
                return Some(&symbol.ty);
            }
        }
        self.parent.as_ref().and_then(|p| p.get(id))
    }

    pub fn get_mut(&mut self, id: SymbolId) -> Option<&mut SymbolStorage> {
        for symbol in &mut self.symbols {
            if symbol.id == id {
                return Some(symbol);
            }
        }
        self.parent.as_mut().and_then(|p| p.get_mut(id))
    }

    pub fn get_parent(&self, n: usize) -> Option<&Scope> {
        let mut current = self;
        for _ in 0..n {
            if let Some(parent) = &current.parent {
                current = parent;
            } else {
                return None;
            }
        }
        Some(current)
    }
}

#[derive(Default, Debug, Clone)]
pub struct SymbolResolver {
    global_scope: Scope,
    current_scope_idx: usize,
    expected_ret_ty: Option<Type>,
}

impl SymbolResolver {
    pub fn new() -> SymbolResolver {
        SymbolResolver {
            global_scope: Scope::new_root(),
            current_scope_idx: 0,
            expected_ret_ty: None,
        }
    }

    pub fn enter_scope(&mut self) {
        self.current_scope_idx += 1;
        self.global_scope = self.global_scope.create_child();
    }

    pub fn exit_scope(&mut self) {
        if self.current_scope_idx > 0 {
            self.current_scope_idx -= 1;
            if let Some(parent) = self.global_scope.get_parent(self.current_scope_idx) {
                self.global_scope = parent.clone();
            }
        }
    }

    pub fn insert(&mut self, id: SymbolId, ty: Type, span: Span) {
        self.global_scope.insert(id, ty, span);
    }

    pub fn get(&self, id: SymbolId) -> Option<&Type> {
        self.global_scope.get(id)
    }

    pub fn get_mut(&mut self, id: SymbolId) -> Option<&mut SymbolStorage> {
        self.global_scope.get_mut(id)
    }
}

impl Iterator for SymbolResolver {
    type Item = Scope;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_scope_idx == 0 {
            return None;
        }
        let scope = self.global_scope.get_parent(self.current_scope_idx);
        scope.cloned()
    }
}

impl SemanticLayer for SymbolResolver {
    type Output = Type;

    fn handle(
        layer_id: usize,
        analyzer: &mut SemanticAnalyzer,
        node: LangNodeId,
    ) -> LayerOutput<Self::Output> {
        let lang_node = analyzer.ast.get(node).clone();
        match lang_node.kind {
            LangNodeKind::Expr(Expr::Ident(id)) => {
                if let Some(ss) = analyzer.layers[layer_id]
                    .symbol_resolver_mut()
                    .get_mut(id.0)
                {
                    analyzer.hir.incr_used(node.into());
                    analyzer.hir.set_type(node.into(), ss.ty.clone());
                    ss.used_count += 1; // Increment usage count
                    LayerOutput::ResOk(ss.ty.clone())
                } else {
                    LayerOutput::ResErr({
                        let symbol = analyzer.ast.get_symbol(id.0);
                        SemanticError::symbol_not_found(symbol.name.clone(), lang_node.span)
                    })
                }
            }
            LangNodeKind::Let(l) => {
                let ty = analyzer.analyze(l.value)?.unwrap();
                analyzer.layers[layer_id].symbol_resolver_mut().insert(
                    l.name.0,
                    ty.clone(),
                    lang_node.span,
                );
                analyzer.hir.set_type(node.into(), ty.clone());
                LayerOutput::ResOk(ty)
            }
            LangNodeKind::FunctionDef {
                name,
                params,
                ret,
                body,
                ..
            } => {
                let mut param_types = Vec::new();
                for param in params.clone() {
                    param_types.push(param.ty.clone());
                }

                let ty = Type::Function(param_types, Box::new(ret.clone()));

                analyzer.layers[layer_id].symbol_resolver_mut().insert(
                    name.0,
                    ty.clone(),
                    lang_node.span,
                );
                analyzer.hir.set_type(node.into(), ty.clone());
                analyzer.layers[layer_id]
                    .symbol_resolver_mut()
                    .expected_ret_ty = Some(ret.clone());
                if let Some(body) = body {
                    analyzer.layers[layer_id]
                        .symbol_resolver_mut()
                        .enter_scope();
                    for param in params.iter() {
                        analyzer.layers[layer_id].symbol_resolver_mut().insert(
                            param.name.0,
                            param.ty.clone(),
                            lang_node.span,
                        );
                    }
                    analyzer.analyze(body)?;
                    analyzer.layers[layer_id].symbol_resolver_mut().exit_scope();
                }

                LayerOutput::ResOk(ty)
            }
            LangNodeKind::Return(ret) => {
                if let Some(expected) = &analyzer.layers[layer_id]
                    .symbol_resolver_mut()
                    .clone()
                    .expected_ret_ty
                {
                    let ret_ty = analyzer.analyze(ret)?.unwrap();
                    if ret_ty != *expected {
                        return LayerOutput::ResErr(SemanticError::type_mismatch(
                            expected.to_string(),
                            ret_ty.to_string(),
                            lang_node.span,
                        ));
                    }
                    analyzer.layers[layer_id]
                        .symbol_resolver_mut()
                        .expected_ret_ty = None; // Reset after return
                } else {
                    return LayerOutput::ResErr(SemanticError::return_not_in_function(
                        lang_node.span,
                    ));
                }
                LayerOutput::Handled
            }
            LangNodeKind::Block(elts) => {
                analyzer.layers[layer_id]
                    .symbol_resolver_mut()
                    .enter_scope();
                for elt in elts {
                    analyzer.analyze(elt)?;
                }
                analyzer.layers[layer_id].symbol_resolver_mut().exit_scope();
                LayerOutput::Handled
            }
            _ => LayerOutput::NotHandled,
        }
    }
}
