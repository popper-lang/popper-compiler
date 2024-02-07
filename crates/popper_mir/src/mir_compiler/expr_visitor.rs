use popper_ast::{BinOp, BinOpKind, Call, Constant, Expression, ParenGroup, StructFieldAccess, StructInstance, UnaryOp};
use popper_ast::visitor::ExprVisitor;
use crate::mir_ast::{Add, BodyFn, Const, List, MirFloat, MirInt, MirString, Value, Variable};
use crate::mir_compiler::MirCompiler;

impl ExprVisitor for MirCompiler {
    type Output = Value;
    type Error = ();

    fn visit_constant(&mut self, constant: Constant) -> Result<Self::Output, Self::Error> {
        Ok(match constant {
            Constant::Ident(ident) => {
                Value::Variable(
                    Variable::new(ident.name.clone(), self.get(ident.name.as_str()).unwrap().clone())
                )
            },
            Constant::Int(int) => {
                Value::Const(
                    Const::Int(MirInt::new(int.value))
                )
            },
            Constant::Float(float) => {
                Value::Const(
                    Const::Float(
                        MirFloat::new(float.value)
                    )
                )
            },
            Constant::StringLiteral(string) => {
                Value::Const(
                    Const::String(
                        MirString::new(string.value)
                    )
                )
            },
            Constant::Null(_) => {
                Value::Const(
                    Const::Void
                )
            },

            _ => unimplemented!()

        })
    }

    fn visit_bin_op(&mut self, bin_op: BinOp) -> Result<Self::Output, Self::Error> {
        let lhs = self.visit_expr(*bin_op.lhs)?;
        let rhs = self.visit_expr(*bin_op.rhs)?;
        let lhs_ty = lhs.get_type();
        let out = self.new_var_id(lhs_ty.clone())?;
        let body = self.current_fn.as_mut().unwrap();

        match bin_op.op {
            BinOpKind::Add => {
                body.push(
                    BodyFn::Add(
                        Add::new(
                            out.clone(),
                            lhs,
                            rhs,

                        )
                    )
                );
            },
            _ => unimplemented!()
        }

        Ok(Value::Variable(
            Variable::new(out, lhs_ty)
        ))
    }

    fn visit_unary_op(&mut self, unary_op: UnaryOp) -> Result<Self::Output, Self::Error> {
        todo!()
    }

    fn visit_group(&mut self, group: ParenGroup) -> Result<Self::Output, Self::Error> {
        todo!()
    }

    fn visit_expr(&mut self, expr: Expression) -> Result<Self::Output, Self::Error> {
        match expr {
            Expression::Constant(constant) => {
                self.visit_constant(constant)
            },
            Expression::BinOp(bin_op) => {
                self.visit_bin_op(bin_op)
            },
            Expression::UnaryOp(unary_op) => {
                self.visit_unary_op(unary_op)
            },
            Expression::Group(group) => {
                self.visit_group(group)
            },
            Expression::Call(call) => {
                self.visit_call(call)
            },
            Expression::StructInstance(struct_instance) => {
                self.visit_struct_instance(struct_instance)
            },
            Expression::StructFieldAccess(struct_field_access) => {
                self.visit_struct_field_access(struct_field_access)
            },
        }
    }

    fn visit_call(&mut self, call: Call) -> Result<Self::Output, Self::Error> {
        if self.current_fn.is_none() {
            return Err(());
        }
        let name = call.name.clone();
        let func = self.get(name.as_str()).expect(
            &format!("Function {} not found", name)
        ).clone();
        let out = self.new_var_id(func.clone())?;
        let args = call.arguments.iter().map(|arg| self.visit_expr(arg.clone())).collect::<Result<Vec<Value>, ()>>()?;

        let list = List::new(args);
        let current_fn = self.current_fn.as_mut().unwrap();

        current_fn.push(
            BodyFn::Call(
                crate::mir_ast::Call::new(
                    name,
                    list,
                    out.clone()
                )
            )
        );

        Ok(Value::Variable(
            Variable::new(out, func)
        ))
    }

    fn visit_struct_instance(&mut self, struct_instance: StructInstance) -> Result<Self::Output, Self::Error> {
        todo!()
    }

    fn visit_struct_field_access(&mut self, struct_field_access: StructFieldAccess) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
