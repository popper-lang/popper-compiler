mod let_stmt;
mod block_stmt;
mod function_stmt;
mod import_stmt;
mod return_stmt;

use crate::compiler::LLVMCompiler;
use popper_ast::Statement;

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
            Statement::Import(import_stmt) => self.compile_import_stmt(import_stmt),
            Statement::Return(return_stmt) => self.compile_return(return_stmt),
            e => todo!("Statement not implemented: {:?}", e)
        }
    }

    pub fn compile_expr_stmt(&mut self, expr: Expression) {
        self.compile_expr(expr);
    }
}