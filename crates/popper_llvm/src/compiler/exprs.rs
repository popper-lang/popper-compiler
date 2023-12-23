use crate::compiler::LLVMCompiler;
use crate::object::pop_object::PopObject;
use popper_ast::Constant;
use popper_ast::BinOp;
use popper_ast::ParenGroup;
use popper_ast::BinOpKind;
use popper_ast::Block;
use popper_ast::Expression;


impl<'ctx> LLVMCompiler<'ctx> {
    pub fn compile_paren_group(&self, paren_group: ParenGroup) -> PopObject {
        self.compile_expr(*paren_group.expr)
    }

    pub fn compile_bin_op(&self, bin_op: BinOp) -> PopObject {
        let lhs = self.compile_expr(*bin_op.lhs);
        let rhs = self.compile_expr(*bin_op.rhs);

        match bin_op.op {
            BinOpKind::Add => {
                match (lhs, rhs) {
                    (PopObject::Int(_, v1), PopObject::Int(_, v2)) => {
                        PopObject::Int(v1.get_type(), self.builder.build_int_add(v1, v2, "add").unwrap())
                    },
                    (PopObject::Float(_, v1), PopObject::Float(_, v2)) => {
                        PopObject::Float(v1.get_type(), self.builder.build_float_add(v1, v2, "add").unwrap())
                    },
                    _ => todo!("BinOp not implemented")
                }
            },
            BinOpKind::Sub => {
                match (lhs, rhs) {
                    (PopObject::Int(_, v1), PopObject::Int(_, v2)) => {
                        PopObject::Int(v1.get_type(), self.builder.build_int_sub(v1, v2, "sub").unwrap())
                    },
                    (PopObject::Float(_, v1), PopObject::Float(_, v2)) => {
                        PopObject::Float(v1.get_type(), self.builder.build_float_sub(v1, v2, "sub").unwrap())
                    },
                    _ => todo!("BinOp not implemented")
                }
            },
            _ => todo!()
        }
    }

    pub fn compile_expr(&self, expr: Expression) -> PopObject {
        match expr {
            Expression::Constant(constant) => self.compile_constant(constant),
            Expression::Group(paren_group) => self.compile_paren_group(paren_group),
            Expression::BinOp(binop) => self.compile_bin_op(binop),

            _ => todo!("Expression not implemented")
        }
    }
    
}