use crate::errors::*;
use crate::value::Value;
use crate::vm::Evaluateur;
use crate::vm::Vm;
use crate::ast::Expr;

#[derive(Clone)]
pub struct IfThen {
    pub cond: Box<Expr>,
    pub then: Box<Expr>
}

impl Evaluateur for IfThen {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        let condition = self.cond.eval(vm)?;
        if let Value::Bool(true) = condition {
            self.then.eval(vm)
        } else {
            Ok(Value::None)
        }
    }
}