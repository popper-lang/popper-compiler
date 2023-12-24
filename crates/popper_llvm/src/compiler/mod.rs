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

    pub fn register_builtin(&mut self) {
        let i32_type = self.context.i32_type();
        let i8_type = self.context.i8_type();

        let printf_type = i32_type.fn_type(&[i8_type.array_type(4).into()], true);
        let printf_func = self.module.add_function("printf", printf_type, None);
        let sign = i32_type.fn_type(&[i32_type.into()], false);


        let func = self.module.add_function(
            "print",
            sign,
            None,
        );

        self.env.set("print".to_string(), PopPointer::from_value(func.as_global_value().as_pointer_value()));

        let basic_block = self.context.append_basic_block(func, "entry");
        let arg_value = func.get_first_param().unwrap();
        let format_string = self.context.const_string("%d\n".to_string().into_bytes().as_slice(), true);
        self.builder.position_at_end(basic_block);
        self.builder.build_call(printf_func, &[format_string.into(), arg_value.into()], "printf_call");
        self.builder.build_return(
            Some(
                &BasicValueEnum::IntValue(
                    self.context.i32_type().const_int(0, false)
                )
            )
        );
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
        self.module.print_to_string().to_string()
    }

    pub fn compile(&mut self, ast: Vec<Statement>) -> String {
        for stmt in ast {
            self.compile_stmt(stmt);
        }
        self.build()
    }

}
