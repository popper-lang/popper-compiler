use popper_ast::If;
use popper_ast::visitor::StmtVisitor;
use crate::compiler::Compiler;

impl StmtVisitor for Compiler {
    fn visit_if_stmt(&mut self, if_stmt: If) -> Result<Self::Output, Self::Error> {

    }
}