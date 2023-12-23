use popper_ast::{BinOp, Call, Constant, Expression, ParenGroup, UnaryOp};
use popper_ast::visitor::ExprVisitor;
use crate::bytecode;
use crate::bytecode::Bytecode;
use crate::compiler::Compiler;

impl ExprVisitor for Compiler {

    type Output = ();
    type Error = String;
    fn visit_constant(&mut self, constant: Constant) -> Result<Self::Output, Self::Error> {
        match constant {
            Constant::Int(int) => {
                self.bytecode.add_instruction(
                    Bytecode::new(
                        bytecode::Instruction::LOAD_CONST,
                        vec![bytecode::BytecodeConstant::Int(int.value as i32)],
                    ))
            },
            Constant::Float(float) => {
                self.bytecode.add_instruction(
                    Bytecode::new(
                        bytecode::Instruction::LOAD_CONST,
                        vec![bytecode::BytecodeConstant::Float(float.value as f32)],
                    ))
            },
            Constant::StringLiteral(string) => {
                self.bytecode.add_instruction(
                    Bytecode::new(
                        bytecode::Instruction::LOAD_CONST,
                        vec![bytecode::BytecodeConstant::String(string.value)],
                    ))
            },
            Constant::Bool(boolean) => {
                self.bytecode.add_instruction(
                    Bytecode::new(
                        bytecode::Instruction::LOAD_CONST,
                        vec![bytecode::BytecodeConstant::Int(boolean.value.into())],
                    ))
            },
            Constant::Null(_) => {
                self.bytecode.add_instruction(
                    Bytecode::new(
                        bytecode::Instruction::LOAD_CONST,
                        vec![bytecode::BytecodeConstant::Null],
                    ))
            },
            Constant::Ident(ident) => {
                self.bytecode.add_instruction(
                    Bytecode::new(
                        bytecode::Instruction::LOAD_VAR,
                        vec![bytecode::BytecodeConstant::String(ident.name)],
                    ))
            },
        }
        Ok(())
    }
    fn visit_group(&mut self, group: ParenGroup) -> Result<Self::Output, Self::Error> {
        self.visit_expr(*group.expr)
    }

    fn visit_bin_op(&mut self, bin_op: BinOp) -> Result<Self::Output, Self::Error> {
        self.visit_expr(*bin_op.lhs)?;
        self.visit_expr(*bin_op.rhs)?;
        match bin_op.op {
            popper_ast::BinOpKind::Add => {
                self.bytecode.add_instruction(
                    Bytecode::new(
                        bytecode::Instruction::OP_ADD,
                        vec![],
                    ))
            },
            popper_ast::BinOpKind::Sub => {
                self.bytecode.add_instruction(
                    Bytecode::new(
                        bytecode::Instruction::OP_SUB,
                        vec![],
                    ))
            },
            popper_ast::BinOpKind::Mul => {
                self.bytecode.add_instruction(
                    Bytecode::new(
                        bytecode::Instruction::OP_MUL,
                        vec![],
                    ))
            },
            popper_ast::BinOpKind::Div => {
                self.bytecode.add_instruction(
                    Bytecode::new(
                        bytecode::Instruction::OP_DIV,
                        vec![],
                    ))
            },
            popper_ast::BinOpKind::Mod => {
                self.bytecode.add_instruction(
                    Bytecode::new(
                        bytecode::Instruction::OP_MOD,
                        vec![],
                    ))
            },
            popper_ast::BinOpKind::Pow => {
                self.bytecode.add_instruction(
                    Bytecode::new(
                        bytecode::Instruction::OP_POW,
                        vec![],
                    ))
            },
            popper_ast::BinOpKind::Eq => {
                self.bytecode.add_instruction(
                    Bytecode::new(
                        bytecode::Instruction::CMP_EQ,
                        vec![],
                    ))
            },
            popper_ast::BinOpKind::Neq => {
                self.bytecode.add_instruction(
                    Bytecode::new(
                        bytecode::Instruction::CMP_NEQ,
                        vec![],
                    ))
            },
            popper_ast::BinOpKind::Gt => {
                self.bytecode.add_instruction(
                    Bytecode::new(
                        bytecode::Instruction::CMP_GT,
                        vec![],
                    ))
            },
            popper_ast::BinOpKind::Lt => {
                self.bytecode.add_instruction(
                    Bytecode::new(
                        bytecode::Instruction::CMP_LT,
                        vec![],
                    ))
            },
            popper_ast::BinOpKind::Gte => {
                self.bytecode.add_instruction(
                    Bytecode::new(
                        bytecode::Instruction::CMP_GTE,
                        vec![],
                    ))
            },
            popper_ast::BinOpKind::Lte => {
                self.bytecode.add_instruction(
                    Bytecode::new(
                        bytecode::Instruction::CMP_LTE,
                        vec![],
                    ))
            },
            popper_ast::BinOpKind::And => {
                self.bytecode.add_instruction(
                    Bytecode::new(
                        bytecode::Instruction::OP_AND,
                        vec![],
                    ))
            },
            popper_ast::BinOpKind::Or => {
                self.bytecode.add_instruction(
                    Bytecode::new(
                        bytecode::Instruction::OP_OR,
                        vec![],
                    ))
            },
        }

        Ok(())
    }

    fn visit_unary_op(&mut self, unary_op: UnaryOp) -> Result<Self::Output, Self::Error> {
        self.visit_expr(*unary_op.expr)?;
        match unary_op.op {
            popper_ast::UnaryOpKind::Not => {
                self.bytecode.add_instruction(
                    Bytecode::new(
                        bytecode::Instruction::OP_NOT,
                        vec![],
                    ))
            },
            popper_ast::UnaryOpKind::Neg => {
                self.bytecode.add_instruction(
                    Bytecode::new(
                        bytecode::Instruction::OP_NEG,
                        vec![],
                    ))
            },
        }

        Ok(())
    }

    fn visit_call(&mut self, call: Call) -> Result<Self::Output, Self::Error> {
        for arg in call.arguments {
            self.visit_expr(arg)?;
        }
        self.bytecode.add_instruction(
            Bytecode::new(
                bytecode::Instruction::CALL_FUNC,
                vec![bytecode::BytecodeConstant::String(call.name), bytecode::BytecodeConstant::Int(call.arguments.len() as i32)],
            ));

        Ok(())
    }

    fn visit_expr(&mut self, expr: Expression) -> Result<Self::Output, Self::Error> {
        match expr {
            Expression::Constant(constant) => self.visit_constant(constant),
            Expression::Group(group) => self.visit_group(group),
            Expression::BinOp(bin_op) => self.visit_bin_op(bin_op),
            Expression::UnaryOp(unary_op) => self.visit_unary_op(unary_op),
            Expression::Call(call) => self.visit_call(call),
        }
    }

}

