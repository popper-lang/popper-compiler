use std::collections::HashMap;
use crate::ast::Expr;
use crate::vm::Evaluateur;
use crate::vm::Vm;
use crate::value::Value;
use crate::value::Type;
use crate::value::Var;
use super::ident::Ident;
use crate::errors::*;

#[derive(Clone)]
pub struct CallStruct {
    pub name: String,
    pub args: Vec<(Ident, Expr)>,
}

impl Evaluateur for CallStruct {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        let copy_vm = vm.clone();
        match copy_vm.get_ident(Ident(self.name.clone())) {
            Some(f) => match *f {
                Var{value: Value::DefStruct {
                    ref fields,
                    ..
                }, ..} => {
                    let mut map = HashMap::new();
                    let mut a ;
                    let mut _v;
                    for (arg, value) in self.args.clone() {
                        a = match arg {
                            Ident(ident) => ident.clone(),
                            _ => {
                                return Err(Error::TypeMismatch(TypeMismatchError {
                                    expected: Type::None,
                                    found: Type::None,
                                }))
                            }
                        };
                        _v = value.eval(vm)?;
                        for field in fields {
                            let Ident(f) = field.clone();
                            if f == a {
                                map.insert(field.clone(), value.eval(vm)?);
                            }

                        }
                    }
                    Ok(Value::CallStruct {
                        name: self.name.clone(),
                        fields: map,
                    })
                }
                _ => Err(Error::TypeMismatch(TypeMismatchError {
                    expected: Type::Struct(self.name.clone()),
                    found: f.value.get_type(),
                })),
            },
            None => Err(Error::StructNotFound(StructNotFoundError {
                name: self.name.clone(),
            })),
        }
    }
}