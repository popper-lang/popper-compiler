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
pub struct GetModFunc {
    pub mod_name: String,
    pub func_name: String,
    pub args: Vec<Expr>
}

impl Evaluateur for GetModFunc {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        let mod_name = self.mod_name.clone();
        let func_name = self.func_name.clone();
        let value = match vm.get_ident(Ident(mod_name.clone())) {
            Some(var) => var,
            None => return Err(Error::VarNotFound(VarNotFoundError { var_name: mod_name })),
        };
        match value {
            Var {
                value: Value::Module { context, .. },
                ..
            } => {
                let var = match context.get(&Ident(func_name.clone())) {
                    Some(var) => var,
                    None => return Err(Error::FunctionNotFound(FunctionNotFoundError {
                        name: func_name,
                    })),
                };
                match var.clone() {
                    Var {value: Value::Function {
                        args,
                        func,
                        ..
                    }, ..} => {
                        let mut args = args.clone();
                        let mut args_map = HashMap::new();
                        for arg in self.args.iter().zip(args.iter()) {
                            let value = arg.0.eval(vm)?;
                            if value.get_type() != arg.1.1.clone() {
                                return Err(Error::TypeMismatch(TypeMismatchError {
                                    expected: arg.1.1.clone(),
                                    found: value.clone().get_type(),
                                }));
                            }

                            args_map.insert(arg.1.0.clone(), Var{value: value.clone(), type_: value.clone().get_type(), mutable: false });
                        }
                        let mut vm = vm.clone();
                        func.0(args_map, vm)

                    },
                    _ => return Err(Error::TypeMismatch(TypeMismatchError {
                        expected: Type::Function,
                        found: var.value.get_type(),
                    }))
                }
            }
            _ => Err(Error::AttrNotFound(crate::errors::AttrNotFoundError {
                attr_name: func_name.clone(),
            })),
        }
    }
}