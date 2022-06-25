use crate::vm::Evaluateur;
use crate::vm::Vm;
use crate::errors::*;
use crate::value::Value;
use crate::value::Type;

#[derive(Clone)]
pub struct TypeExpr(pub Type);

impl Evaluateur for TypeExpr {
    fn eval(&self, _vm: &mut Vm) -> Result<Value, Error> {
        Ok(Value::Type(self.0.clone()))
    }
}