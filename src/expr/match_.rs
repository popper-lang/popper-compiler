use crate::vm::Vm;
use crate::vm::Evaluateur;
use crate::errors::*;
use crate::value::Value;
use crate::ast::Expr;

#[derive(Clone)]
pub struct Match {
    pub value: Box<Expr>,
    pub cases: Vec<(Expr, Expr)>,
}

impl Evaluateur for Match {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        let mut return_value = Value::None;
        let value = self.value.eval(vm)?;
        for i in self.cases.clone() {
            let _case = i.0.eval(vm)?;
            if _case == value {
                return_value = i.1.eval(vm)?;
            }
        }
        Ok(return_value)
    }
}