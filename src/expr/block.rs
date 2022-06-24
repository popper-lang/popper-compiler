use crate::vm::Evaluateur;
use crate::vm::Vm;
use crate::value::Value;
use crate::errors::*;
use crate::ast::Expr;

#[derive(Clone)]
pub struct Block {
    pub body: Vec<Expr>,
}

impl Evaluateur for Block {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        let mut last = Value::None;
        for expr in self.body.clone() {
            last = expr.eval(vm)?;
        }
        Ok(last)
    }
}