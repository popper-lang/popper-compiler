use crate::errors::Error;
use crate::value::Value;
use crate::vm::Evaluateur;
use crate::vm::Vm;
use crate::ast::Expr;

#[derive(Clone)]
pub struct IfThenElse {
    pub cond: Box<Expr>,
    pub then: Box<Expr>,
    pub else_: Box<Expr>,
}

impl Evaluateur for IfThenElse {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        let condition = self.cond.eval(vm)?;
        if let Value::Bool(true) = condition {
            self.then.eval(vm)
        } else {
            self.else_.eval(vm)
        }
    }
}
