

mod object;

use std::rc::Rc;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use inkwell::basic_block::BasicBlock;
pub use inkwell::context::Context;
use inkwell::builder::Builder;
use inkwell::module::Module;
use inkwell::types::{BasicMetadataTypeEnum, BasicTypeEnum};
use inkwell::values::{FloatValue, IntValue};
use inkwell::values::AnyValueEnum::InstructionValue;
use popper_ast::{BinOp, Call, Constant, Expression, If, LetStmt, ParenGroup, Statement, Type, TypeKind, UnaryOp};
use popper_ast::visitor::{ExprVisitor, StmtVisitor};
use object::*;
use popper_ast::Block;

// represent the llvm code generated with Rust type
pub struct LLVMGeneratedCode<'ctx, T> {
    pub result: T,
    pub module: Module<'ctx>,
    pub context: &'ctx Context
}

pub struct LLVMError<'ctx> {
    message: &'ctx str
}

pub struct LLVMEnv<'ctx> {
    var: HashMap<String, Pointer<'ctx>>
}

impl<'ctx> LLVMEnv<'ctx> {
    pub fn new() -> LLVMEnv<'ctx> {
        LLVMEnv {
            var: HashMap::new()
        }
    }

    pub fn get(&self, name: String) -> Option<&Pointer> {
        self.var.get(&name)
    }

    pub fn set(&mut self, name: String, ptr: Pointer<'ctx>) {
        self.var.insert(name, ptr);
    }
}




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

    pub fn compile_int(&self, value: i32) -> PopObject {
        let ty = self.context.i32_type();
        let int_value = ty.const_int(value as u64, false);
        PopObject::Int(ty, int_value)
    }

    pub fn compile_float(&self, value: f32) -> PopObject {
        let ty = self.context.f32_type();
        let float_value = ty.const_float(value as f64);
        PopObject::Float(ty, float_value)
    }

    pub fn compile_string(&self, value: String) -> PopObject {
        let cstring = std::ffi::CString::new(value).expect("Cast failed");
        let bytes: &[u8] = cstring.as_bytes_with_nul();
        let array_value = self.context.const_string(bytes, false);
        PopObject::String(PopString::from_array_value(array_value))
    }

    pub fn compile_bool(&self, value: bool) -> PopObject {
        let ty = self.context.bool_type();
        let bool_value = ty.const_int(value as u64, false);
        PopObject::Bool(ty, bool_value)
    }

    pub fn compile_constant(&self, constant: Constant) -> PopObject {
        match constant {
            Constant::Int(int) => self.compile_int(int.value as i32),
            Constant::Float(float) => self.compile_float(float.value as f32),
            Constant::StringLiteral(string) => self.compile_string(string.value),
            Constant::Bool(boolean) => self.compile_bool(boolean.value),
            Constant::Ident(ident) => {
                let ptr = self.env.get(ident.name).unwrap();
                let ptr = ptr.value;
                let ty = ptr.get_type();
                let val = self.builder.build_load(ty, ptr, "load").unwrap();
                PopObject::from_basic_value_enum(val)
            },
            _ => todo!("Constant not implemented")
        }
    }

    pub fn compile_paren_group(&self, paren_group: ParenGroup) -> PopObject {
        self.compile_expr(*paren_group.expr)
    }

    pub fn compile_expr(&self, expr: Expression) -> PopObject {
        match expr {
            Expression::Constant(constant) => self.compile_constant(constant),
            Expression::Group(paren_group) => self.compile_paren_group(paren_group),
            _ => todo!("Expression not implemented")
        }
    }

    pub fn compile_block(&mut self, block: Block) -> BasicBlock {
        let old_basic_block = self.current_basic_block;
        let llvm_block = self.context.append_basic_block(self.module.get_function(self.current_function).unwrap(),
                                                         format!(
                                                             "{}_block_{}",
                                                             self.current_function,
                                                             self.module.get_function(self.current_function).unwrap().get_basic_blocks().len()
                                                         ).as_str());
        self.builder.position_at_end(llvm_block);
        self.current_basic_block = Some(llvm_block);
        for stmt in block.statements {
            self.compile_stmt(stmt);
        }
        self.current_basic_block = old_basic_block;
        llvm_block
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

    pub fn compile_let(&mut self, let_stmt: LetStmt) {
        let ty = PopType::from_ty_ast(let_stmt.r#type.unwrap().type_kind);
        let value = self.compile_expr(let_stmt.value);
        let value = value.to_basic_value_enum();
        let ptr = self.builder.build_alloca(ty.to_llvm_type(&self.context), "let").unwrap();
        self.builder.build_store(ptr, value).expect("Failed to build store");
        self.env.set(let_stmt.name.name, Pointer::new(ptr.get_type(), ptr));
    }

    pub fn compile_stmt(&mut self, stmt: Statement) {
        match stmt {
            // Statement::If(if_stmt) => self.compile_if(if_stmt),
            Statement::Let(let_stmt) => self.compile_let(let_stmt),
            Statement::Expression(expr) => {
                self.compile_expr_stmt(expr);
            },
            Statement::Block(block) => {
                self.compile_block(block);
            },
            Statement::Function(func) => self.compile_function(func),
            _ => todo!("Statement not implemented")
        }
    }

    pub fn compile_function(&mut self, function: popper_ast::Function) {
        let args = function
            .arguments
            .args
            .iter()
            .map(|x|
                PopType::from_ty_ast(x.ty.clone().type_kind).to_basic_metadata_type(&self.context)
            )
            .collect::<Vec<BasicMetadataTypeEnum>>()
        ;
        let func_ty = PopType::from_ty_ast(function.returntype.type_kind).to_llvm_type(&self.context);

        let func_ty = func_ty.into_int_type().fn_type(args.as_slice(), false);

        let func = self.module.add_function(function.name.as_str(), func_ty, None);

        let block = self.context.append_basic_block(func, "entry");

        self.builder.position_at_end(block);
        for stmt in function.body {
            self.compile_stmt(stmt);
        }
        self.current_basic_block = Some(block);
    }

    pub fn compile_expr_stmt(&mut self, expr: Expression) {
        self.compile_expr(expr);
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

}