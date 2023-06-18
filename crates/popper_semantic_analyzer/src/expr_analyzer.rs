use ast::*;
use crate::errors::TypeMismatch;
use popper_flag::{
    ScopeFlag,
    Flag,
    TypeFlag,
    VariableFlag,
    Environment,
    SymbolFlags,
};

use ast::visitor::ExprVisitor;
use popper_common::error::Error;

pub struct ExprAnalyzer {
    env: Environment,
}


impl ExprAnalyzer {
    pub fn new(env: Environment) -> Self {
        Self { env: env }
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
                    .set_string()
                    .clone()
            ),
            Constant::Bool(bool) => Ok(
                SymbolFlags::new(bool.span())
                    .set_boolean()
                    .clone()
            ),
            Constant::Ident(ident) => {
                match self.env.get_variable(&ident.name) {
                    Some(v) => Ok(
                        SymbolFlags::new(ident.span)
                            .add_flag(
                                *v.value.clone()
                            ).clone()
                    ),
                    None => todo!("throw name not found error")
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

    fn visit_expr(&mut self, expr: Expression) -> Result<Self::Output, Self::Error> {
        match expr {
            Expression::Constant(constant) => self.visit_constant(constant),
            Expression::BinOp(bin_op) => self.visit_bin_op(bin_op),
            Expression::UnaryOp(unary_op) => self.visit_unary_op(unary_op),
            Expression::Group(group) => self.visit_group(group),
        }
    }
}








