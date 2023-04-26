use crate::ast::stmt::{Stmt, StmtType};
use crate::ast::expr::{Expr, ExprType};
use crate::bytecodes::bytecode::{Bytecode, Opcode, Operand};


impl Stmt {

    // `to_bytecode` is used to convert an AST into a bytecode representation.
    pub fn to_bytecode(&self) -> Bytecode {
        let mut bytecode = Bytecode::new();

        match &*self.stmt_type {
            StmtType::Expression { expr } => {
                let expr = expr.to_bytecode();
                bytecode.extend(expr);
            },
            StmtType::If { cond, then } => {
                let cond = cond.to_bytecode();
                let then = then.to_bytecode();
                bytecode.extend(cond);
                bytecode.add_instruction(Opcode::If, Some(Operand::Int(then.instructions.len() as i32)));
                bytecode.add_instruction(Opcode::Jump, Some(Operand::Int((bytecode.instructions.len() + then.instructions.len()) as i32)));
                bytecode.extend(then);
            },
            StmtType::IfElse { cond, then, else_ } => {
                let cond = cond.to_bytecode();
                let then = then.to_bytecode();
                let else_ = else_.to_bytecode();
                bytecode.extend(cond);
                bytecode.add_instruction(Opcode::If, Some(Operand::Int((bytecode.instructions.len() + 1) as i32)));
                bytecode.add_instruction(Opcode::Jump, Some(Operand::Int((bytecode.instructions.len() + then.instructions.len()) as i32)));
                bytecode.extend(then);
                bytecode.extend(else_);
            },
            StmtType::Block { body } => {
                for stmt in body {
                    let stmt = stmt.to_bytecode();
                    bytecode.extend(stmt);
                }
            },
            _ => todo!()
        }

        bytecode
    }
}