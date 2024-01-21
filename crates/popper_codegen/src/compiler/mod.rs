#![allow(dead_code)]


use std::collections::HashMap;
use std::env::var;
use std::path::Path;
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
    used_cdylib: Vec<String>,
    structs: HashMap<String, HashMap<String, u8>>, // struct name -> field name -> index
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
            used_cdylib: Vec::new(),
            structs: HashMap::new(),
        }
    }

    pub fn compile_from_rust_to_dylib(&mut self, path: String) {
        let path = Path::new(&path);
        let hash = popper_common::hash::hash_file(path.to_str().unwrap());
        let mut binding = path.with_extension("");
        binding.set_file_name(
            format!("librust_{}_{}.dylib", binding.file_name().unwrap().to_str().unwrap(), hash).as_str()
        );


        let filename = binding.file_name().unwrap().to_str().unwrap();
        let popper_target_var = var("POPPER_TARGET").unwrap();
        let popper_target_path = Path::new(popper_target_var.as_str());
        let lib_path = popper_target_path.join("libs");
        let dylib_path = lib_path.join(filename);
        if ! lib_path.exists() {
            cmd!(mkdir "-p" popper_target_path.to_str().unwrap());
        }
        cmd!(rustc "--crate-type=dylib" "-o" dylib_path.to_str().unwrap() path.to_str().unwrap());
        self.used_cdylib.push(dylib_path.to_str().unwrap().to_string());
    }

    pub fn compile_from_ll_to_dylib(&mut self, path: String) {
        let path = Path::new(&path);
        let mut binding = path.with_extension("");
        let hash = popper_common::hash::hash_file(path.to_str().unwrap());
        binding.set_file_name(
            format!("libll_{}_{}.dylib", binding.file_name().unwrap().to_str().unwrap(), hash).as_str()
        );
        let filename = binding.file_name().unwrap().to_str().unwrap();
        let popper_target_var = var("POPPER_TARGET").unwrap();
        let popper_target_path = Path::new(popper_target_var.as_str());
        let lib_path = popper_target_path.join("libs");
        let dylib_path = lib_path.join(filename);
        if ! lib_path.exists() {
            cmd!(mkdir "-p" popper_target_path.to_str().unwrap());
        }
        dbg!(dylib_path.to_str().unwrap());

        cmd!(llc "-filetype=obj" "-o" dylib_path.to_str().unwrap() path.to_str().unwrap());
        self.used_cdylib.push(dylib_path.to_str().unwrap().to_string());
    }

    pub fn get_used_cdylib(&self) -> Vec<String> {
        self.used_cdylib.clone()
    }

    pub fn build(&self) -> String {
        self.module.print_to_string().to_string()
    }

    pub fn compile(&mut self, ast: Vec<Statement>) {
        for stmt in ast {
            self.compile_stmt(stmt);
        }
    }

    pub fn set_env(&mut self, name: String, value: PopObject<'ctx>) {
        self.env.set(name, value.clone());
    }

}
