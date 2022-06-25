use crate::errors::Error;
use crate::value::Value;
use crate::vm::{Evaluateur, Vm};

#[derive(Debug, Clone)]
pub enum LiteralType {
    Number(f64),
    String(String),
    Bool(bool),
    None,
}

#[derive(Clone)]
pub struct Literal(pub LiteralType);

impl Evaluateur for Literal {
    fn eval(&self, _vm: &mut Vm) -> Result<Value, Error> {
        Ok(match self.0.clone() {
            LiteralType::Number(n) => Value::Number(n),
            LiteralType::String(s) => Value::String(s.clone()),
            LiteralType::Bool(b) => Value::Bool(b),
            LiteralType::None => Value::None,
        })
    }
}
