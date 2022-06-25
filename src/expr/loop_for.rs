use super::ident::Ident;
use crate::ast::Expr;
use crate::errors::*;
use crate::value::Type;
use crate::value::Value;
use crate::value::Var;
use crate::vm::Evaluateur;
use crate::vm::Vm;

#[derive(Clone)]
pub struct For {
    pub name: String,
    pub iter: Box<Expr>,
    pub body: Box<Expr>,
}

impl Evaluateur for For {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        let iter = self.iter.eval(vm)?;
        match iter {
            Value::List(ref l) => {
                let mut last = Value::None;
                for item in l {
                    vm.set_ident(
                        Ident(self.name.clone()),
                        Var {
                            value: item.clone(),
                            type_: Type::Int,
                            mutable: true,
                        },
                    );
                    last = self.body.eval(vm)?;
                }
                Ok(last)
            }
            Value::Range(r) => {
                let mut last = Value::None;
                for i in r {
                    vm.set_ident(
                        Ident(self.name.clone()),
                        Var {
                            value: Value::Number(i as f64),
                            type_: Type::Int,
                            mutable: true,
                        },
                    );
                    last = self.body.eval(vm)?;
                }
                Ok(last)
            }
            _ => Err(Error::TypeMismatch(TypeMismatchError {
                expected: Type::List,
                found: iter.get_type(),
            })),
        }
    }
}
