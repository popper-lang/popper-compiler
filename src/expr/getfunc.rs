use std::collections::HashMap;

use crate::ast::Expr;
use crate::value::Function;
use crate::value::Var;
use crate::value::Value;
use crate::value::Type;
use crate::vm::Evaluateur;
use crate::vm::Vm;
use crate::errors::*;

use super::ident::Ident;


#[derive(Clone)]
pub struct GetFunc {
    pub name: String,
    pub func: String,
    pub args: Vec<Expr>,
}

impl Evaluateur for GetFunc {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        let call_struct;
        let s = match vm.get_ident(Ident(self.name.clone())) {
            Some(Var {value: Value::CallStruct { name: n, fields: fi }, ..}) => {
                call_struct = Value::CallStruct { name: n.clone(), fields: fi.clone() };
                match &vm.get_ident(Ident(n.clone())) {
                    Some(Var{value: Value::DefStruct { fields: f, function: fu , ..}, ..}) => {
                        match fu.get(&self.func) {
                            Some(v) => v.clone(),
                            None => {
                                return Err(Error::FunctionNotFound(FunctionNotFoundError {
                                    name: self.func.clone(),
                                }))
                            }
                        }
                    }
                    _ => {
                        return Err(Error::TypeMismatch(TypeMismatchError {
                            expected: Type::Struct(self.name.clone()),
                            found: Type::None,
                        }))
                    }
                }
            }
            _ => {
                return Err(Error::TypeMismatch(TypeMismatchError {
                    expected: Type::Struct(self.name.clone()),
                    found: Type::None,
                }))
            }
        };
            
        match s {
            Value::Function {func: f, args: a, ..} => {
                let Function(f) = f;
                let mut new_vm = Vm::new();
                let mut args_map = HashMap::new();
                for (argv, argn) in self.args.iter().zip(a) {
                    
                    let value = argv.eval(vm)?;
                    if value.get_type() != argn.1 {
                        return Err(Error::TypeMismatch(TypeMismatchError {
                            expected: argn.clone().1,
                            found: value.get_type(),
                        }))
                    }
                    args_map.insert(argn.0, Var {value: value.clone(), type_: value.get_type(), mutable: false});
                }
                new_vm.set_ident(Ident("self".to_string()), Var{value: call_struct, type_: Type::Struct(self.name.clone()), mutable: false});

                
                return f(args_map, new_vm);
            },
            _ => {
                return Err(Error::TypeMismatch(TypeMismatchError {
                    expected: Type::Func,
                    found: Type::None,
                }))
            }
        }
    }
}        