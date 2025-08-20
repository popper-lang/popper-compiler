use inkwell::context::Context;
use popper_ast::layer::{Ast, Layer};
use popper_semantic_analyzer::hir::{Hir, HirNodeId};

pub struct PopperCodegenLLVM {
    ctx: Context,
}

impl PopperCodegenLLVM {
    pub fn new() -> Self {
        let ctx = Context::create();
        PopperCodegenLLVM { ctx }
    }
}

impl Layer for PopperCodegenLLVM {
    type Inner = Hir;
    type Output = ();

    fn handle(&mut self, ast: &Hir, node: HirNodeId) -> Self::Output {}
}
