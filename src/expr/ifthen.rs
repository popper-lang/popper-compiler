use crate::ast::Expr;
use crate::errors::*;
use crate::value::Type;
use crate::value::Value;
use crate::vm::Evaluateur;
use crate::vm::Vm;

#[derive(Clone, Debug)]
pub struct IfThen {
    pub cond: Box<Expr>,
    pub then: Box<Expr>,
}

impl Evaluateur for IfThen {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        let condition = self.cond.eval(vm)?;
        
        if let Value::Bool(true) = condition {
            self.then.eval(vm)
        } else if let Value::Bool(false) = condition {
            Ok(Value::None)
        } else {
            Err(Error::TypeMismatch(TypeMismatchError {
                expected: Type::Bool,
                found: condition.get_type(),
            }))
        }
    }
}
