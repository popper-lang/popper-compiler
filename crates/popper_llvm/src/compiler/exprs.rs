use inkwell::values::{BasicMetadataValueEnum, BasicValue};
use crate::compiler::LLVMCompiler;
use crate::object::pop_object::PopObject;

use popper_ast::BinOp;
use popper_ast::ParenGroup;
use popper_ast::BinOpKind;

use popper_ast::Expression;
use popper_ast::Call;
use crate::object::pop_type::PopType;


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
                    (PopObject::Ptr(_, v1), PopObject::Int(_, v2)) => {
                        PopObject::Ptr(v1.get_type(), self.builder.build_int_add(v1, v2.const_to_pointer(v1.get_type()), "add").unwrap())
                    },
                    (PopObject::Int(_, v1), PopObject::Ptr(_, v2)) => {
                        PopObject::Ptr(v2.get_type(), self.builder.build_int_add(v1.const_to_pointer(v2.get_type()), v2, "add").unwrap())
                    },
                    (PopObject::Ptr(_, v1), PopObject::Ptr(_, v2)) => {
                        PopObject::Ptr(v1.get_type(), self.builder.build_int_add(v1, v2, "add").unwrap())
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

    pub fn compile_call(&self, call: Call) -> PopObject {
        let func = self.module.get_function(call.name.as_str()).unwrap();
        let mut args = vec![];
        for arg in call.arguments {
            let arg = self.compile_expr(arg);
            let ptr = arg.cast_to_ptr(self.context, &self.builder);
            args.push(ptr);
        }
        let args: Vec<BasicMetadataValueEnum> = args.iter()
            .map(|arg| arg.as_basic_value_enum())
            .map(|arg| {
                arg.into()
            })
            .collect();
        let _ret = self.builder.build_call(func, args.as_slice(), "call").unwrap();
        PopObject::new_int(self.context, 3)
    }

    pub fn compile_expr(&self, expr: Expression) -> PopObject {
        match expr {
            Expression::Constant(constant) => self.compile_constant(constant),
            Expression::Group(paren_group) => self.compile_paren_group(paren_group),
            Expression::BinOp(binop) => self.compile_bin_op(binop),
            Expression::Call(call) => self.compile_call(call),

            _ => todo!("Expression not implemented")
        }
    }
    
}