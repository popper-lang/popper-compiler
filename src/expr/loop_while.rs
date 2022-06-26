use crate::ast::Expr;
use crate::errors::Error;
use crate::value::Value;
use crate::vm::Evaluateur;
use crate::vm::Vm;

#[derive(Clone, Debug)]
pub struct While {
    pub cond: Box<Expr>,
    pub body: Box<Expr>,
}

impl Evaluateur for While {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        let mut cond = self.cond.eval(vm)?;
        while let Value::Bool(true) = cond {
            self.body.eval(vm)?;
            cond = self.cond.eval(vm)?;
        }
        Ok(Value::None)
    }
}
