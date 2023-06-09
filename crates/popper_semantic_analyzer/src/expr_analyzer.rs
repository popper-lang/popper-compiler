use ast::*;
use crate::error::TypeMismatch;
use crate::symbol_table::{SymbolTable, SymbolFlags, Flag, ConstantType, Type};
use crate::visitor::ExprVisitor;

struct ExprAnalyzer {
    symbol_table: SymbolTable,
}


impl ExprAnalyzer {
    fn new(symbol_table: SymbolTable) -> Self {
        Self { symbol_table }
    }
}

impl ExprVisitor for ExprAnalyzer {
    type Output = SymbolFlags;
    type Error = ();

    fn visit_constant(&mut self, constant: Constant) -> Result<Self::Output, Self::Error> {
        match constant {
            Constant::Int(_) => Ok(
                SymbolFlags::new()
                    .set_integer()
                    .clone()
            ),
            Constant::Float(_) => Ok(
                SymbolFlags::new()
                    .set_float()
                    .clone()
            ),
            Constant::StringLiteral(_) => Ok(
                SymbolFlags::new()
                    .set_string()
                    .clone()
            ),
            Constant::Bool(_) => Ok(
                SymbolFlags::new()
                    .set_boolean()
                    .clone()
            ),
            Constant::Ident(ident) => {
                match self.symbol_table.get(&ident.name) {
                    Some(_) => Ok(
                        SymbolFlags::new()
                            .set_ident()
                            .clone()
                    ),
                    None => Err(()),
                }
            },
            Constant::Null(_) => Ok(
                SymbolFlags::new()
                    .set_none()
                    .clone()
            ),

        }
    }

    fn visit_bin_op(&mut self, bin_op: BinOp) -> Result<Self::Output, Self::Error> {
        let flag_lhs = self.visit_expr(*bin_op.lhs)?;
        let flag_rhs = self.visit_expr(*bin_op.rhs)?;

        if flag_lhs.is_same_type(&flag_rhs) {
            Ok(flag_lhs)
        } else {
            todo!("throw type mismatch error")
        }


    }

    fn visit_unary_op(&mut self, unary_op: UnaryOp) -> Result<Self::Output, Self::Error> {

        let flag_expr = self.visit_expr(*unary_op.expr)?;
        if unary_op.op == UnaryOpKind::Not {
            if flag_expr.has_flag(Flag::Type(Type::Boolean)) {
                Ok(flag_expr)
            } else {
                todo!("throw type mismatch error")
            }
        } else if unary_op.op == UnaryOpKind::Neg {
            if flag_expr.has_flag(Flag::Type(Type::Integer)) || flag_expr.has_flag(Flag::Type(Type::Float)) {
                Ok(flag_expr)
            } else {
                todo!("throw type mismatch error")
            }
        } else {
            todo!("throw type mismatch error")
        }
    }

    fn visit_group(&mut self, group: ParenGroup) -> Result<Self::Output, Self::Error> {
        todo!()
    }

    fn visit_expr(&mut self, expr: Expression) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}








