#![allow(dead_code)]


use std::env::var;
use std::path::Path;
use std::process::Command;

use inkwell::module::Module;
use inkwell::context::Context;
use inkwell::builder::Builder;
use inkwell::basic_block::BasicBlock;




use llvm_env::LLVMEnv;
use popper_ast::{Statement};

use crate::object::pop_object::PopObject;

use crate::cmd;

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

    pub fn load_builtins(&mut self) {
        // let builtins = load_builtins();
        // for builtin in builtins {
        //     let function = builtin.llvm_fn_sign();
        //     let args: Vec<PopType> = function.arguments.args.iter().map(|x| PopType::from_ty_ast(x.ty.type_kind.clone())).collect();
        //     let ret = PopType::from_ty_ast(function.returntype.type_kind);
        //     let llvm_type = ret.to_llvm_type(self.context);
        //     let param_types: Vec<_> = args
        //         .iter()
        //         .map(|x| x.clone().to_llvm_type(self.context))
        //         .map(|x| Into::<BasicMetadataTypeEnum>::into(x))
        //         .collect();
        //
        //     let function_type = llvm_type.fn_type(param_types.as_slice(), false);
        //     let function_value = self.module.add_function(function.name.as_str(), function_type, None);
        //
        // }
    }

    pub fn compile_dylib(&self, path: String) {
        let path = Path::new(&path);
        let binding = path.with_extension("dylib");
        let filename = binding.file_name().unwrap().to_str().unwrap();
        let popper_target_var = var("POPPER_TARGET").unwrap();
        let popper_target_path = Path::new(popper_target_var.as_str());
        let lib_path = popper_target_path.join("libs");
        let dylib_path = lib_path.join(filename);
        if ! lib_path.exists() {
            cmd!(mkdir "-p" popper_target_path.to_str().unwrap());
        }

        cmd!(rustc "--crate-type=dylib" "-o" dylib_path.to_str().unwrap() path.to_str().unwrap());
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
