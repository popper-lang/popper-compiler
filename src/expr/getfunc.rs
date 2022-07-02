use std::collections::HashMap;

use crate::ast::Expr;
use crate::errors::*;
use crate::value::Function;
use crate::value::Object;
use crate::value::Type;
use crate::value::Value;
use crate::value::Var;
use crate::vm::Evaluateur;
use crate::vm::Vm;

use super::ident::Ident;

#[derive(Clone, Debug)]
pub struct GetFunc {
    pub name: Box<Expr>,
    pub func: String,
    pub args: Vec<Expr>,
}

impl Evaluateur for GetFunc {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        let name = self.name.eval(vm)?;
        
        let s = match name.get_object() {
            Object {
                attr,
                ..
            } => {
                let attr = attr.get(&self.func);
                match attr {
                    Some(attr) => {
                        attr.clone().value
                    },
                    None => return Err(Error::FunctionNotFound(FunctionNotFoundError {
                        name: self.func.clone(),
                    })),
                }
            },
        };

        match s {
            Value::Function {
                func: f, args: a, ..
            } => {
                let call_struct = name.clone();
                let Function(f) = f;
                let mut new_vm = Vm::new();
                let mut args_map = HashMap::new();
                for (argv, argn) in self.args.iter().zip(a) {
                    let value = argv.eval(vm)?;
                    if value.get_type() != argn.1 {
                        println!("[ERROR] line 56 file 'getfunc.rs', {:#?}", Error::TypeMismatch(TypeMismatchError {
                            expected: argn.clone().1,
                            found: value.get_type(),
                        }));
                        return Err(Error::TypeMismatch(TypeMismatchError {
                            expected: argn.clone().1,
                            found: value.get_type(),
                        }));
                    }
                    args_map.insert(
                        argn.0,
                        Var {
                            value: value.clone(),
                            type_: value.get_type(),
                            mutable: false,
                        },
                    );
                }
                if let Value::Module { context, .. } = name {
                    let mut b = HashMap::new();
                    for (k, v) in context.iter() {
                        b.insert(match k {
                            Ident(name) => name.clone()
                        }, v.clone());
                    }
                    args_map.extend(b);
                } else {
                    new_vm.set_ident(
                        Ident("self".to_string()),
                        Var {
                            value: call_struct,
                            type_: Type::Struct(name.clone().get_object().name),
                            mutable: false,
                        },
                    );
                    
                }

                return f(args_map, new_vm);
            }
            _ => {
                println!("[ERROR] line 95 file 'getfunc.rs', {:#?}", Error::TypeMismatch(TypeMismatchError {
                    expected: Type::Func,
                    found: s.get_type(),
                }));
                return Err(Error::TypeMismatch(TypeMismatchError {
                    expected: Type::Func,
                    found: Type::None,
                }))
            }
        }
    }
}
