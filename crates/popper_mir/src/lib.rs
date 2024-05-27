#![allow(unused)]


use crate::compiler::Compiler;

pub mod consts;
pub mod expr;
pub mod command;
pub mod stmt;
pub mod types;
pub mod function;
pub mod debug;
pub mod marks;
pub mod labels;
pub mod builder;
pub mod pretty;
pub mod program;
pub mod compiler;
mod bytecode;

pub use program::Program;

pub fn compile(program: Vec<popper_ast::Statement>) -> Program {
    let mut compiler = Compiler::new(program);
    compiler.compile();
    let builder = compiler.get_builder();
    builder.get_program()
}
