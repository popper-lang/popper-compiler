use crate::ast::Expr;
use crate::vm::Vm;
use crate::vm::Evaluateur;
use crate::errors::*;
use crate::value::Value;
use crate::value::Type;

#[derive(Clone)]
pub struct To {
    pub value: Box<Expr>,
    pub type_: Type,
}

impl Evaluateur for To{
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        let v = self.value.eval(vm)?;
        match self.type_ {
            Type::Int => {
                match v {
                    Value::Number(i) => Ok(Value::Number(i)),
                    Value::String(s) => {
                        Ok(Value::Number(match s.parse::<i32>() {
                            Ok(i) => i,
                            Err(_) => {
                                return Err(Error::InvalidCastNumber(InvalidCastNumberError {
                                    elt: s.clone()
                                }))
                            }
                        } as f64))
                    },
                    _ => Err(Error::TypeMismatch(TypeMismatchError {
                        expected: Type::Int,
                        found: v.get_type(),
                    })),
                }
            },
            Type::String => {
                match v {
                    Value::String(s) => Ok(Value::String(s)),
                    Value::Number(i) => Ok(Value::String(i.to_string())),
                    _ => Err(Error::TypeMismatch(TypeMismatchError {
                        expected: Type::None,
                        found: v.get_type(),
                    })),
                }
            },
            Type::Bool => {
                match v {
                    Value::Bool(b) => Ok(Value::Bool(b)),
                    _ => Err(Error::TypeMismatch(TypeMismatchError {
                        expected: Type::Bool,
                        found: v.get_type(),
                    })),
                }
            },
            _ => Err(Error::TypeMismatch(TypeMismatchError {
                expected: Type::None,
                found: v.get_type(),
            })), 
        }
    }        
}