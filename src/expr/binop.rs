use crate::ast::Expr;
use crate::ast::Op;
use crate::errors::Error;
use crate::value::Value;
use crate::vm::Evaluateur;
use crate::vm::Vm;

#[derive(Clone, Debug)]
pub struct BinOp {
    pub op: Op,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

impl Evaluateur for BinOp {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        let left = self.left.eval(vm)?;
        let right = self.right.eval(vm)?;
        match self.op {
            Op::Add => Ok(left.add(&right)?),
            Op::Sub => Ok(left.sub(&right)?),
            Op::Mul => Ok(left.mul(&right)?),
            Op::Div => Ok(left.div(&right)?),
            Op::Mod => Ok(left.modulo(&right)?),
            Op::Pow => Ok(left.pow(&right)?),
            Op::Eq => Ok(left.eq(&right)?),
            Op::Neq => Ok(left.neq(&right)?),
            Op::Lt => Ok(left.lt(&right)?),
            Op::Gt => Ok(left.gt(&right)?),
            Op::Le => Ok(left.le(&right)?),
            Op::Ge => Ok(left.ge(&right)?),
            Op::And => Ok(left.and(&right)?),
            Op::Or => Ok(left.or(&right)?),
        }
    }
}
