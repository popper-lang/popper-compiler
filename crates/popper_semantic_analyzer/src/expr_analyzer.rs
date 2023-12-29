use popper_ast::*;
use popper_error::{diff_length_of_argument::DiffLengthOfArgument, namenotfound::NameNotFound, typemismatch::TypeMismatch};
use popper_flag::{Environment, SymbolFlags, ValueFlag};


use popper_ast::visitor::ExprVisitor;
use popper_error::Error;
use popper_common::name_similarity::find_similar_name;




#[derive(Clone)]
pub struct ExprAnalyzer {
    env: Environment,
}


impl ExprAnalyzer {
    pub fn new(env: Environment) -> Self {
        Self { env }
    }

    pub fn get_type(&self, ty: Type) -> ValueFlag {
        match ty.type_kind {
            TypeKind::Bool => ValueFlag::Boolean,
            TypeKind::Float => ValueFlag::Float,
            TypeKind::Int => ValueFlag::Integer,
            TypeKind::String(size) => ValueFlag::String(size),
            TypeKind::Array(ty, _) => ValueFlag::Array(Box::new(self.get_type(*ty))),
            TypeKind::Function(args, returnty) => {
                let mut args_type = Vec::new();
                for arg in args {
                    args_type.push(self.get_type(arg));
                }
                ValueFlag::Function(args_type, Box::new(self.get_type(*returnty)))
            }
            TypeKind::Unit => ValueFlag::None,
            TypeKind::Pointer(ptr) => ValueFlag::Pointer(
                Box::new(self.get_type(*ptr))
                ),
            _ => unimplemented!()
        }
    }
}

impl ExprVisitor for ExprAnalyzer {
    type Output = SymbolFlags;
    type Error = Box<dyn Error>;

    fn visit_constant(&mut self, constant: Constant) -> Result<Self::Output, Self::Error> {
        match constant {
            Constant::Int(int) => Ok(
                SymbolFlags::new(int.span())
                    .set_integer()
                    .clone()
            ),
            Constant::Float(float) => Ok(
                SymbolFlags::new(float.span())
                    .set_float()
                    .clone()
            ),
            Constant::StringLiteral(string) => Ok(
                SymbolFlags::new(string.span())
                    .set_string(string.value.len() as u32)
                    .clone()
            ),
            Constant::Bool(bool) => Ok(
                SymbolFlags::new(bool.span())
                    .set_boolean()
                    .clone()
            ),
            Constant::Ident(ref ident) => {
                match self.env.get_variable(&ident.name) {
                    Some(v) => Ok({
                        let mut s = SymbolFlags::new(ident.span);
                        s.symbols.extend(v.value.symbols.clone());
                        s
                    }),
                    None => {
                         let name_candidates = self.env.get_all_variables_name();

                        let similar_name = find_similar_name(name_candidates.as_slice(), ident.name.as_str());

                        Err(
                            Box::new(
                                NameNotFound::new(
                                    (ident.span, ident.name.clone()),
                                    similar_name.cloned()
                                )

                            )
                        )
                    }
                }
            },
            Constant::Null(null) => Ok(
                SymbolFlags::new(null.span())
                    .set_none()
                    .clone()
            ),

        }
    }

    fn visit_bin_op(&mut self, bin_op: BinOp) -> Result<Self::Output, Self::Error> {
        let flag_lhs = self.visit_expr(*bin_op.lhs)?;
        let flag_rhs = self.visit_expr(*bin_op.rhs)?;
        if flag_lhs.is_same_value(flag_rhs.clone()) {
            Ok(flag_lhs)
        } else {
            Err(
                Box::new(
                    TypeMismatch::new(
                        (flag_lhs.clone().span(), flag_lhs.get_value().unwrap().to_string()),
                        (flag_rhs.clone().span(), flag_rhs.get_value().unwrap().to_string())
                    )
                )
            )
        }


    }

    fn visit_unary_op(&mut self, unary_op: UnaryOp) -> Result<Self::Output, Self::Error> {

        let flag_expr = self.visit_expr(*unary_op.expr)?;
        if unary_op.op == UnaryOpKind::Not {
            if flag_expr.clone().is_boolean() {
                Ok(flag_expr)
            } else {
                Err(
                    Box::new(
                        TypeMismatch::new(
                            (flag_expr.span, "boolean".to_string()),
                            (flag_expr.span, flag_expr.get_value().unwrap().to_string())
                        )
                    )
                )
            }
        } else if unary_op.op == UnaryOpKind::Neg {
            if flag_expr.is_integer() || flag_expr.is_float() {
                Ok(flag_expr)
            } else {
                Err(
                    Box::new(
                        TypeMismatch::new(
                            (flag_expr.span, "Integer or Float".to_string()),
                            (flag_expr.span, flag_expr.get_value().unwrap().to_string())
                        )
                    )
                )
            }
        } else {
            unreachable!()
        }
    }

    fn visit_group(&mut self, group: ParenGroup) -> Result<Self::Output, Self::Error> {
        self.visit_expr(*group.expr)
    }

    fn visit_call(&mut self, call: Call) -> Result<Self::Output, Self::Error> {
        let x = self.env.get_variable(&call.name);

        match x {
            Some(var) => {
                match var.value.get_function() {
                    Some((args, ret)) => {
                        let args_s = call.arguments.iter().map(|arg| self.clone().visit_expr(arg.clone())).collect::<Result<Vec<_>, _>>()?;
                        if args_s.len() != args.len() {
                            return Err(
                                Box::new(
                                    DiffLengthOfArgument::new(args.len(), args_s.len(), call.span)
                                )
                            );
                        }
                        for (arg_get, arg_model) in args_s.iter().zip(args) {
                            let arg_get_value: ValueFlag = arg_get.get_value().unwrap();
                            let arg_model_value: ValueFlag = arg_model.clone();
                            if arg_get_value != arg_model_value  {
                                return Err(
                                    Box::new(
                                        TypeMismatch::new(
                                            (call.span, arg_model_value.to_string()),
                                            (call.span, arg_get_value.to_string())
                                        )
                                    )
                                );
                            }
                        }
                        Ok(SymbolFlags::new(call.span).set_value(*ret.clone()).clone())
                    },
                    None => {
                        Err(
                            Box::new(
                                TypeMismatch::new(
                                    (call.span, "function".to_string()),
                                    (call.span, var.value.get_value().unwrap().to_string())
                                )
                            )
                        )
                    }
                }
            },
            None => {
                let name_candidates = self.env.get_all_variables_name();
                let similar_name = find_similar_name(name_candidates.as_slice(), call.name.as_str());

                Err(
                    Box::new(
                        NameNotFound::new(
                            (call.span, call.name.clone()),
                            similar_name.cloned()
                        )

                    )
                )
            }
        }
    }

    fn visit_expr(&mut self, expr: Expression) -> Result<Self::Output, Self::Error> {
        match expr {
            Expression::Constant(constant) => self.visit_constant(constant),
            Expression::BinOp(bin_op) => self.visit_bin_op(bin_op),
            Expression::UnaryOp(unary_op) => self.visit_unary_op(unary_op),
            Expression::Group(group) => self.visit_group(group),
            Expression::Call(call) => self.visit_call(call),
        }
    }


}








