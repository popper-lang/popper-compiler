

use super::SbCompiler;
use popper_ast::*;
use visitor::ExprVisitor;
use crate::value::ByteStr; // This is the ExprVisitor from popper_ast::visitor

impl ExprVisitor for SbCompiler {
    type Output = ();
    type Error = ();

    fn visit_bin_op(&mut self, bin_op: BinOp) -> Result<Self::Output, Self::Error> {
        let _ = self.visit_expr(*bin_op.lhs);
        let _ = self.visit_expr(*bin_op.rhs);

        match bin_op.op {
            BinOpKind::Add => {
                self.ir.emit_add();
            }
            BinOpKind::Sub => {
                self.ir.emit_sub();
            }
            BinOpKind::Mul => {
                self.ir.emit_mul();
            }
            BinOpKind::Div => {
                self.ir.emit_div();
            }
            _ => todo!("bin op")

        }

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
            Constant::StringLiteral(StringLiteral  { value , ..}) => {
                self.ir.emit_string(ByteStr::new(value));
            }
            Constant::Bool(boolean) => {
                self.ir.emit_bool(boolean.value);
            }
            Constant::Ident(ident) => {
                let ident = ident.name;
                self.ir.emit_variable(
                    ByteStr::new(ident)
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