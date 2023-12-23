use crate::bytecode;

mod expr_compiler;
mod stmt_compiler;


#[derive(Clone, Debug, PartialEq)]
pub struct Compiler {
    pub bytecode: bytecode::Bytecodes,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            bytecode: bytecode::Bytecodes::new(),
        }
    }

    pub fn compile(&mut self, ast: Vec<popper_ast::Statement>) {
        for stmt in &ast {
            stmt_compiler::compile_stmt(stmt, &mut self.bytecode);
        }
    }
}