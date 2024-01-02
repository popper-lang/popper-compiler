use crate::compiler::LLVMCompiler;
use popper_ast::ImportStmt;
use popper_common::ast_path_to_path::ast_path_to_path;
use crate::compiler::llvm_env::LLVMEnv;
use crate::cmd;
use std::env::var;
use std::path::Path;

impl<'ctx> LLVMCompiler<'ctx> {
    pub fn compile_import_stmt(&mut self, import_stmt: ImportStmt) {
        let path = ast_path_to_path(import_stmt.path);
        let mut llvm_compiler = LLVMCompiler::new(self.context, LLVMEnv::new(), "");

        for stmt in import_stmt.module_stmts {
            llvm_compiler.compile_stmt(stmt);
        }

        self.env.extend(llvm_compiler.env.clone());
        llvm_compiler.module.get_functions().for_each(|func| {
            self.module.add_function(func.get_name().to_str().unwrap(), func.get_type(), None);
        });

        let binding = var("POPPER_TARGET").unwrap();

        let popper_target_path= Path::new(&binding);
        let ll_path = popper_target_path.join(path.with_extension("ll").file_name().unwrap());
        let build = llvm_compiler.build();
        self.used_cdylib.extend(llvm_compiler.used_cdylib.clone());
        cmd!(touch ll_path.to_str().unwrap());
        std::fs::write(ll_path.clone(), build).unwrap();
        self.compile_from_ll_to_dylib(ll_path.to_str().unwrap().to_string());
        cmd!(rm ll_path.to_str().unwrap());
    }
}