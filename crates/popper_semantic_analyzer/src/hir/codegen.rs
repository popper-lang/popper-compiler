use popper_ast::ast::{LangAst, LangNodeId};
use popper_ast::layer::{Ast, Layer};
use crate::hir::Hir;

pub struct HirCodegen {
    hir: Hir,
    ast: LangAst
}

impl HirCodegen {
    pub fn new(ast: LangAst) -> Self {
        let hir = Hir::create_from_ast(&ast);
        HirCodegen { hir, ast }
    }
    
    pub fn codegen_node(&mut self, node_id: LangNodeId) {}
}


pub struct HirCodegenLayer;

impl Layer for HirCodegenLayer {
    type Inner = LangAst;
    type Output = Hir;

    fn handle(&mut self, ast: &Self::Inner, node: LangNodeId) -> Self::Output {
        let mut codegen = HirCodegen::new(ast.clone());
        codegen.codegen_node(node);
        codegen.hir
    }
}