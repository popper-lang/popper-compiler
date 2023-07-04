
use super::SbCompiler;
use popper_ast::*;
use visitor::{StmtVisitor, ExprVisitor};
use crate::instr::Instruction;
use crate::value::StrPtr;

impl StmtVisitor for SbCompiler {
    type Error = ();
    type Output = ();

    fn visit_block(&mut self, block: Block) -> Result<Self::Output, Self::Error> {
        for stmt in block.statements {
            let _ = self.visit_stmt(stmt);
        }
        Ok(())
    }

    fn visit_expr_stmt(&mut self, expr: Expression) -> Result<Self::Output, Self::Error> {
        let _ = self.visit_expr(expr)?;
        self.ir.emit_pop();
        Ok(())
    }

    fn visit_let_stmt(&mut self, let_stmt: LetStmt) -> Result<Self::Output, Self::Error> {

        let name = let_stmt.name.name;
        let len = name.len();
        let name = name.as_ptr();
        self.visit_expr(let_stmt.value)?;
        self.ir.emit_store(
            StrPtr::new(name, len)
        );

        Ok(())
    }

    fn visit_while_stmt(&mut self, while_stmt: While) -> Result<Self::Output, Self::Error> {
        self.visit_expr(while_stmt.condition)?;
        let start = self.ir.instructions.len();
        self.ir.emit_jump_if_false_included(0);
        self.visit_stmt(*while_stmt.body)?;
        self.ir.emit_jump_included(start.clone()-1);
        self.ir.replace_instruction(start,
                                    Instruction::JIFIncluded(
                                        self.ir.instructions.len()
                                    )
        );
        self.ir.emit_nop();
        Ok(())
    }

    fn visit_stmt(&mut self, stmt: Statement) -> Result<Self::Output, Self::Error> {
        match stmt {
            Statement::Expression(expr) => {
                self.visit_expr_stmt(expr)?;
            }
            Statement::Let(let_stmt) => {
                self.visit_let_stmt(let_stmt)?;
            }
            Statement::While(while_stmt) => {
                self.visit_while_stmt(while_stmt)?;
            }
            Statement::Block(block) => {
                self.visit_block(block)?;
            }
        }
        Ok(())
    }
}