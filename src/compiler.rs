use crate::ast::stmt::{Stmt, StmtType};
use crate::ast::expr::{Expr, ExprType};
use crate::bytecodes::bytecode::{Bytecode, Opcode, Operand, StrPtr};


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
            StmtType::Let { name, value, mutable, type_ } => {
                let opcode = if let Some(expr) = value {
                    if *mutable {
                        Opcode::StoreMut
                    } else {
                        Opcode::Store
                    }
                } else {
                    if *mutable {
                        Opcode::InitMut
                    } else {
                        Opcode::Init
                    }
                };

                if let Some(expr) = value {
                    let expr = expr.to_bytecode();
                    bytecode.extend(expr);
                }
                let name_ptr = StrPtr {
                    ptr: name.lexeme.as_ptr(),
                    len: name.lexeme.len()
                };

                bytecode.add_instruction(opcode, Some(Operand::Str(name_ptr)));
            },
            StmtType::Block { body } => {
                for stmt in body {
                    let stmt = stmt.to_bytecode();
                    bytecode.extend(stmt);
                }
            },
            StmtType::Function { name, args, body } => {
                let body = body.to_bytecode();
                let name_ptr = StrPtr {
                    ptr: name.lexeme.as_ptr(),
                    len: name.lexeme.len()
                };

                for arg in args {
                    let arg_ptr = StrPtr {
                        ptr: arg.as_ptr(),
                        len: arg.len()
                    };
                    bytecode.add_instruction(Opcode::LoadConst, Some(Operand::Str(arg_ptr)));
                }

                bytecode.add_instruction(Opcode::LoadConst, Some(Operand::Str(name_ptr)));
                bytecode.add_instruction(Opcode::LoadConst, Some(
                        Operand::Int(args.len() as i32)
                    )
                );
                bytecode.add_instruction(Opcode::StoreFunc, Some(
                    Operand::Int(
                        (bytecode.instructions.len() + 3) as i32
                    )
                ));
                bytecode.add_instruction(Opcode::Jump, Some(Operand::Int((bytecode.instructions.len() + body.instructions.len() + 3) as i32)));
                let mut body = body.clone();
                if Opcode::Return != body.last_opcode().unwrap_or(Opcode::EndOfProgram) {
                    body.add_instruction(Opcode::Return, None);
                }

                bytecode.extend(body);



            },
            _ => todo!()
        }

        bytecode
    }
}