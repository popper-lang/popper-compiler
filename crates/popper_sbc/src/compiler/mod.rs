#[allow(dead_code)]

mod expr_compiler;
mod stmt_compiler;

use crate::instr::Instruction;
use crate::ir_sb::SbcIr;
use popper_ast::Statement;
use popper_ast::Expression;
use popper_ast::visitor::StmtVisitor;
use popper_ast::visitor::ExprVisitor;

#[derive(Clone)]
pub struct SbCompiler {
    pub ir: SbcIr,
    to_add_at_end: Vec<Instruction>
}

impl SbCompiler {
    pub(crate) fn build_stmt(stmt: Statement) -> Vec<Instruction> {
        let mut compiler = Self::new();
        compiler.visit_stmt(stmt).unwrap();
        compiler.build()
    }

    pub(crate) fn build_expr(expr: Expression) -> Vec<Instruction> {
        let mut compiler = Self::new();
        compiler.visit_expr(expr).unwrap();
        compiler.build()
    }
    pub fn new() -> Self {
        Self {
            ir: SbcIr::new(),
            to_add_at_end: Vec::new()
        }
    }
    
    pub fn build(self) -> Vec<Instruction> {
        let mut ir = self.ir;
        
        ir.instructions.extend(self.to_add_at_end);
        ir.instructions
    }

}
