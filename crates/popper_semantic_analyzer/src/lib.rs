#![feature(try_trait_v2)]

use crate::hir::Hir;
use crate::symbol_resolver::SymbolResolver;
use crate::type_checker::TypeChecker;
use popper_ast::ast::{LangAst, LangNodeId};
use popper_ast::layer::Layer;
use popper_ast::type_::Type;
use std::convert::Infallible;
use std::ops::{ControlFlow, FromResidual, Try};

use std::cell::RefCell;
use std::rc::Rc;

mod error;
pub mod hir;
pub mod symbol_resolver;
pub mod type_checker;
mod typed;

pub enum LayerOutput<T> {
    NotHandled,
    Handled,
    ResOk(T),
    ResErr(error::SemanticError),
}

impl<T> LayerOutput<T> {
    pub fn is_not_handled(&self) -> bool {
        matches!(self, LayerOutput::NotHandled)
    }
    pub fn is_handled(&self) -> bool {
        matches!(self, LayerOutput::Handled)
    }
    pub fn is_ok(&self) -> bool {
        matches!(self, LayerOutput::ResOk(_))
    }

    pub fn is_err(&self) -> bool {
        matches!(self, LayerOutput::ResErr(_))
    }

    pub fn unwrap(self) -> T {
        match self {
            LayerOutput::ResOk(t) => t,
            _ => panic!("Called unwrap on an error result"),
        }
    }

    pub fn err(self) -> error::SemanticError {
        match self {
            LayerOutput::ResErr(e) => e,
            _ => panic!("Called err on a successful result"),
        }
    }
}

impl<T> FromResidual for LayerOutput<T> {
    fn from_residual(residual: error::SemanticError) -> Self {
        LayerOutput::ResErr(residual)
    }
}

impl<T> FromResidual<Result<Infallible, error::SemanticError>> for LayerOutput<T> {
    fn from_residual(residual: Result<Infallible, error::SemanticError>) -> Self {
        match residual {
            Ok(_) => LayerOutput::NotHandled,
            Err(e) => LayerOutput::ResErr(e),
        }
    }
}

impl<T> Try for LayerOutput<T> {
    type Output = Option<T>;
    type Residual = error::SemanticError;

    fn from_output(output: Self::Output) -> Self {
        match output {
            Some(t) => LayerOutput::ResOk(t),
            None => LayerOutput::NotHandled,
        }
    }

    fn branch(self) -> ControlFlow<error::SemanticError, Option<T>> {
        match self {
            LayerOutput::ResOk(t) => ControlFlow::Continue(Some(t)),
            LayerOutput::ResErr(e) => ControlFlow::Break(e),
            LayerOutput::NotHandled => ControlFlow::Continue(None),
            LayerOutput::Handled => ControlFlow::Continue(None),
        }
    }
}

impl<T> From<Result<T, error::SemanticError>> for LayerOutput<T> {
    fn from(result: Result<T, error::SemanticError>) -> Self {
        match result {
            Ok(t) => LayerOutput::ResOk(t),
            Err(e) => LayerOutput::ResErr(e),
        }
    }
}

pub trait SemanticLayer {
    type Output;

    fn handle(
        layer_id: usize,
        analyzer: &mut SemanticAnalyzer,
        node: LangNodeId,
    ) -> LayerOutput<Self::Output>;
}

#[derive(Debug, Clone)]
pub enum SemanticLayerKind {
    TypeChecker(TypeChecker),
    SymbolResolver(SymbolResolver),
}

impl SemanticLayerKind {
    pub fn handle(
        &self,
        layer_id: usize,
        analyzer: &mut SemanticAnalyzer,
        node: LangNodeId,
    ) -> LayerOutput<Type> {
        match self {
            SemanticLayerKind::TypeChecker(_) => TypeChecker::handle(layer_id, analyzer, node),
            SemanticLayerKind::SymbolResolver(_) => {
                SymbolResolver::handle(layer_id, analyzer, node)
            }
        }
    }

    pub fn type_checker_mut(&mut self) -> &mut TypeChecker {
        match self {
            SemanticLayerKind::TypeChecker(t) => t,
            _ => {
                panic!("Semantic layers is not type checker");
            }
        }
    }

    pub fn symbol_resolver_mut(&mut self) -> &mut SymbolResolver {
        match self {
            SemanticLayerKind::SymbolResolver(s) => s,
            _ => panic!("Semantic layer is not symbol resolver"),
        }
    }
}

pub struct SemanticAnalyzer {
    layers: Vec<SemanticLayerKind>,
    ast: LangAst,
    hir: Hir,
}

impl SemanticAnalyzer {
    pub fn new(ast: LangAst) -> Self {
        SemanticAnalyzer {
            layers: vec![],
            hir: Hir::create_from_ast(&ast),
            ast,
        }
    }

    pub fn add_type_checker_layer(&mut self, t: TypeChecker) {
        self.layers.push(SemanticLayerKind::TypeChecker(t));
    }

    pub fn add_symbol_resolver_layer(&mut self, layer: symbol_resolver::SymbolResolver) {
        self.layers.push(SemanticLayerKind::SymbolResolver(layer));
    }

    pub fn analyze(&mut self, node_id: LangNodeId) -> error::Result<Option<Type>> {
        let mut handled = false;
        let mut ty = None;

        for i in 0..self.layers.len() {
            let result = self.layers[i].clone().handle(i, self, node_id);
            match result {
                LayerOutput::NotHandled => continue,
                LayerOutput::Handled => {
                    handled = true;
                    continue;
                }
                LayerOutput::ResOk(t) => {
                    handled = true;
                    if ty.as_ref() != Some(&t) && ty.is_some() {
                        panic!(
                            "Layer {:?} returned a different type than the previous layer",
                            i
                        );
                    }
                    ty = Some(t);
                }
                LayerOutput::ResErr(e) => {
                    return Err(e);
                }
            }
        }
        /*
        a + b
         */
        if !handled {
            panic!(
                "No layer handled the node with id: {:?} node: {:?} (it is a bug in the semantic analyzer)",
                node_id,
                self.ast.get(node_id)
            );
        }

        Ok(ty)
    }
}

pub struct SemanticAnalyzerLayer;

impl Layer for SemanticAnalyzerLayer {
    type Inner = LangAst;
    type Output = error::Result<()>;

    fn handle(&mut self, ast: &LangAst, node: LangNodeId) -> Self::Output {
        let mut semantic_analyzer = SemanticAnalyzer::new(ast.clone());
        semantic_analyzer.add_type_checker_layer(type_checker::TypeChecker::new());
        semantic_analyzer.add_symbol_resolver_layer(symbol_resolver::SymbolResolver::new());
        semantic_analyzer.analyze(node).map(|_| ())
    }
}
