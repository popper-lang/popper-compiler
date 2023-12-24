use crate::compiler::LLVMCompiler;
use popper_ast::ImportStmt;
use popper_module::stmt_path_to_path;
use popper_module::ModuleLoader;
use popper_module::StdModuleLoader;

impl<'ctx> LLVMCompiler<'ctx> {
    pub fn compile_import_stmt(&mut self, import_stmt: ImportStmt) {
        if import_stmt.path.segments.first().unwrap().name == "stdlib" {
            self.ext += &StdModuleLoader.load(import_stmt.path.clone()).unwrap();
        } else {
            todo!("importing from other modules is not supported yet")
        }
    }
}