use popper_ast::*;
use crate::errors::{NameNotFound, TypeMismatch};
use popper_flag::{
    ScopeFlag,
    Flag,
    TypeFlag,
    VariableFlag,
    Environment,
    SymbolFlags,
};

use crate::tool::name_similarity::find_similar_name;

use popper_ast::visitor::ExprVisitor;
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
            Constant::Ident(ref ident) => {
                match self.env.get_variable(&ident.name) {
                    Some(v) => Ok({
                        let mut s = SymbolFlags::new(ident.span);
                        s.symbols.extend(v.value.symbols.clone());
                        s
                    }),
                    None => {
                         let name_candidates = self.env.get_all_variables_name();

                        let similar_name = find_similar_name(name_candidates.as_slice().clone(), ident.name.as_str());

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

    fn visit_expr(&mut self, expr: Expression) -> Result<Self::Output, Self::Error> {
        match expr {
            Expression::Constant(constant) => self.visit_constant(constant),
            Expression::BinOp(bin_op) => self.visit_bin_op(bin_op),
            Expression::UnaryOp(unary_op) => self.visit_unary_op(unary_op),
            Expression::Group(group) => self.visit_group(group),
        }
    }
}








