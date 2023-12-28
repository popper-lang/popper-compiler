use crate::compiler::LLVMCompiler;
use popper_ast::ImportStmt;
impl<'ctx> LLVMCompiler<'ctx> {
    pub fn compile_import_stmt(&mut self, _import_stmt: ImportStmt) {
        todo!()
    }
}