use inkwell::module::Module;
use inkwell::context::Context;
use inkwell::builder::Builder;
use inkwell::basic_block::BasicBlock;
use inkwell::types::BasicMetadataTypeEnum;
use llvm_env::LLVMEnv;
use popper_ast::{BinOp, BinOpKind, Block, Constant, Expression, LetStmt, ParenGroup, Statement};
use crate::object::pop_object::PopObject;
use crate::object::pop_pointer::PopPointer;
use crate::object::pop_string::PopString;
use crate::object::pop_type::PopType;

pub mod llvm_env;mod constants;
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
            filename
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
        self.module.print_to_string().to_string()
    }

    pub fn compile(&mut self, ast: Vec<Statement>) -> String {
        for stmt in ast {
            self.compile_stmt(stmt);
        }
        self.build()
    }

}
