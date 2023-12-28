#![allow(dead_code)]

use std::path::Path;

use inkwell::module::Module;
use inkwell::context::Context;
use inkwell::builder::Builder;
use inkwell::basic_block::BasicBlock;
use inkwell::memory_buffer::MemoryBuffer;



use llvm_env::LLVMEnv;
use popper_ast::{Statement};
use crate::object::pop_object::PopObject;




pub mod llvm_env;
mod constants;
mod exprs;
mod stmts;


#[derive(Debug)]
pub struct LLVMCompiler<'ctx> {
    context: &'ctx Context,
    env: LLVMEnv<'ctx>,
    builder: Builder<'ctx>,
    module: Module<'ctx>,
    current_basic_block: Option<BasicBlock<'ctx>>,
    current_function: &'ctx str,
    filename: &'ctx str,
}

impl<'ctx> LLVMCompiler<'ctx> {

    pub fn new(context: &'ctx Context, env: LLVMEnv<'ctx>, filename: &'ctx str) -> LLVMCompiler<'ctx> {
        let builder = context.create_builder();
        let module = context.create_module(filename);
        let current_basic_block = None;
        let current_function = "main";


        LLVMCompiler {
            context,
            env,
            builder,
            module,
            current_basic_block,
            current_function,
            filename,
        }
    }

    pub fn load_module(&self) {
        let path = Path::new("stdlib");
        path.read_dir().unwrap().for_each(|x| {
            let path = x.unwrap().path();
            let _filename = path.file_name().unwrap().to_str().unwrap();
            let buffer = MemoryBuffer::create_from_file(&path).unwrap();
            let module = self.context.create_module_from_ir(buffer).unwrap();
            self.module.link_in_module(module).unwrap();
        });
    }

    pub fn build(&self) -> String {
        self.module.print_to_string().to_string()
    }

    pub fn compile(&mut self, ast: Vec<Statement>) -> String {
        for stmt in ast {
            self.compile_stmt(stmt);
        }
        self.build()
    }

    pub fn set_env(&mut self, name: String, value: PopObject<'ctx>) {
        self.env.set(name, value.clone());
    }

}
