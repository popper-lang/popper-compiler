

use super::SbCompiler;
use popper_ast::*;
use visitor::ExprVisitor;
use crate::value::StrPtr; // This is the ExprVisitor from popper_ast::visitor

impl ExprVisitor for SbCompiler {
    type Output = ();
    type Error = ();

    fn visit_bin_op(&mut self, bin_op: BinOp) -> Result<Self::Output, Self::Error> {
        let _ = self.visit_expr(*bin_op.lhs);
        let _ = self.visit_expr(*bin_op.rhs);

        self.ir.emit_add();

        Ok(())
    }

    fn visit_constant(&mut self, constant: Constant) -> Result<Self::Output, Self::Error> {

        match constant {
            Constant::Int(int) => {
                self.ir.emit_int(int.value);
            }
            Constant::Float(float) => {
                self.ir.emit_float(float.value);
            }
            Constant::StringLiteral(string) => {
                let string = string.value;
                let len = string.len();
                let string = string.as_ptr();
                self.ir.emit_string(StrPtr::new(string, len));
            }
            Constant::Bool(boolean) => {
                self.ir.emit_bool(boolean.value);
            }
            Constant::Ident(ident) => {
                let ident = ident.name;
                let len = ident.len();
                let ident = ident.as_ptr();

                self.ir.emit_variable(
                    StrPtr::new(ident, len)
                );
            }
            Constant::Null(_) => {
                self.ir.emit_null();
            }
        }

        Ok(())
    }

    fn visit_unary_op(&mut self, unary_op: UnaryOp) -> Result<Self::Output, Self::Error> {
        let _ = self.visit_expr(*unary_op.expr);
        self.ir.emit_neg();
        Ok(())
    }

    fn visit_expr(&mut self, expr: Expression) -> Result<Self::Output, Self::Error> {
        match expr {
            Expression::BinOp(bin_op) => {
                self.visit_bin_op(bin_op)?;
            }
            Expression::Constant(constant) => {
                self.visit_constant(constant)?;
            }
            Expression::UnaryOp(unary_op) => {
                self.visit_unary_op(unary_op)?;
            }
            Expression::Group(group) => {
                self.visit_group(group)?;
            }
        }

        Ok(())
    }

    fn visit_group(&mut self, group: ParenGroup) -> Result<Self::Output, Self::Error> {
        self.visit_expr(*group.expr)?;
        Ok(())
    }
}