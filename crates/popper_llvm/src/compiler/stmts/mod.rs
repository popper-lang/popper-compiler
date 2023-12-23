mod let_stmt;
mod block_stmt;
mod function_stmt;

use crate::compiler::LLVMCompiler;
use popper_ast::Statement;
use popper_ast::Block;
use popper_ast::Expression;


impl<'ctx> LLVMCompiler<'ctx> {
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

    pub fn compile_expr_stmt(&mut self, expr: Expression) {
        self.compile_expr(expr);
    }
}