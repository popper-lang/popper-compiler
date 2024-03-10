#![allow(unused)]


use crate::compiler::Compiler;

mod consts;
mod expr;
mod command;
mod stmt;
mod types;
mod function;
mod debug;
mod marks;
mod labels;
mod builder;
mod pretty;
mod program;
mod compiler;

pub use crate::program::Program;

pub fn compile(program: Vec<popper_ast::Statement>) -> Program {
    let mut compiler = Compiler::new(program);
    compiler.compile();
    let builder = compiler.get_builder();
    builder.get_program()
}