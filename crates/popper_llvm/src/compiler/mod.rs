use inkwell::AddressSpace;
use inkwell::module::Module;
use inkwell::context::Context;
use inkwell::builder::Builder;
use inkwell::basic_block::BasicBlock;
use inkwell::values::{BasicValueEnum, IntValue};

use llvm_env::LLVMEnv;
use popper_ast::{Statement};

use crate::object::pop_pointer::PopPointer;



pub mod llvm_env;
mod constants;
mod exprs;
mod stmts;


pub struct LLVMCompiler<'ctx> {
    context: &'ctx Context,
    env: LLVMEnv<'ctx>,
    builder: Builder<'ctx>,
    module: Module<'ctx>,
    current_basic_block: Option<BasicBlock<'ctx>>,
    current_function: &'ctx str,
    filename: &'ctx str,
    ext: String
}

impl<'ctx> LLVMCompiler<'ctx> {
    pub fn new(context: &'ctx Context, env: LLVMEnv<'ctx>, filename: &'ctx str) -> LLVMCompiler<'ctx> {
        let builder = context.create_builder();
        let module = context.create_module(filename);
        let current_basic_block = None;
        let current_function = "main";
        let ext = "".to_string();

        LLVMCompiler {
            context,
            env,
            builder,
            module,
            current_basic_block,
            current_function,
            filename,
            ext
        }
    }






    // pub fn compile_if(&mut self, if_stmt: If) {
    //
    //     let block = if let Statement::Block(block) = *if_stmt.body {
    //         block
    //     } else {
    //         Block::new(if_stmt.span, vec![*if_stmt.body])
    //     };
    //
    //     let then_block = self.compile_block(block);
    //
    //     // no else block
    //     self.builder.build_conditional_branch(
    //         self.compile_expr(if_stmt.condition)
    //             .to_basic_value_enum()
    //             .into_int_value(),
    //         then_block,
    //         self.current_basic_block
    //     ).expect("Failed to build conditional branch");
    // }

    pub fn build(&self) -> String {
        self.builder.build_return(
            Some(&BasicValueEnum::IntValue(
                self.context.i32_type().const_int(0, false)
            ))
        );
        self.ext.clone() + &self.module.print_to_string().to_string()
    }

    pub fn compile(&mut self, ast: Vec<Statement>) -> String {
        for stmt in ast {
            self.compile_stmt(stmt);
        }
        self.build()
    }

}
