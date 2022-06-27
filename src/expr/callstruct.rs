use super::ident::Ident;
use crate::ast::Expr;
use crate::errors::*;
use crate::value::Type;
use crate::value::Value;
use crate::value::Var;
use crate::vm::Evaluateur;
use crate::vm::Vm;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct CallStruct {
    pub name: String,
    pub args: Vec<(Ident, Expr)>,
}

impl Evaluateur for CallStruct {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        let copy_vm = vm.clone();
        match copy_vm.get_ident(Ident(self.name.clone())) {
            Some(f) => match *f {
                Var {
                    value: Value::DefStruct { ref fields, .. },
                    ..
                } => {
                    let mut map = HashMap::new();
                    let mut v;
                    for (arg, value) in self.args.clone() {
                        let Ident(a) = arg;
                        v = value.eval(vm)?;
                        for field in fields {
                            let Ident(f) = field.clone().0;
                            if f == a {
                                if v.get_type() != field.1 {
                                    return Err(Error::TypeMismatch(TypeMismatchError {
                                        expected: field.clone().1,
                                        found: v.get_type(),
                                    }));
                                }
                                map.insert(field.0.clone(), value.eval(vm)?);
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
