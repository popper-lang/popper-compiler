use std::collections::HashMap;

use crate::ast::Expr;
use crate::errors::*;
use crate::value::Function;
use crate::value::Type;
use crate::value::Value;
use crate::value::Var;

use crate::vm::Evaluateur;
use crate::vm::Vm;

use super::ident::Ident;

#[derive(Clone, Debug)]
pub struct Call {
    pub name: String,
    pub args: Vec<Expr>,
}

impl Evaluateur for Call {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        let copy_vm = vm.clone();
        match copy_vm.get_ident(Ident(self.name.clone())) {
            Some(f) => match f.clone() {
                Var {
                    value: Value::Function { func, args: a, .. },
                    ..
                } => {
                    let mut dict_args = HashMap::new();
                    for (i, arg) in a.iter().enumerate() {
                        let value = self.args[i].eval(vm)?;

                        if value.get_type() != arg.1 && arg.1 != Type::Any {
                            return Err(Error::TypeMismatch(TypeMismatchError {
                                expected: arg.clone().1,
                                found: value.get_type(),
                            }));
                        }
                        dict_args.insert(
                            arg.0.clone(),
                            Var {
                                value: value.clone(),
                                type_: value.get_type(),
                                mutable: false,
                            },
                        );
                    }

                    let Function(f) = func;
                    f(dict_args, vm.clone())
                }
                _ => Err(Error::TypeMismatch(TypeMismatchError {
                    expected: Type::Func,
                    found: f.value.get_type(),
                })),
            },
            None => Err(Error::FunctionNotFound(FunctionNotFoundError {
                name: self.name.clone(),
            })),
        }
    }
}
