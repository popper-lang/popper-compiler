use crate::ast::Expr;
use crate::errors::*;
use crate::value::Value;
use crate::vm::Evaluateur;
use crate::vm::Vm;

#[derive(Clone, Debug)]
pub struct Typeof {
    pub value: Box<Expr>,
}

impl Evaluateur for Typeof {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        let value = self.value.eval(vm)?;
        Ok(Value::Type(value.get_type()))
    }
}
