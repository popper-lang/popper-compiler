

use std::collections::HashMap;
use std::rc::Rc;
use crate::std_t::Builtin;

use crate::errors::*;
use crate::value::Value;
use crate::value::Function;
use crate::expr::ident::Ident;
use crate::value::Var;
use crate::value::Type;
use crate::std_t::BuiltinFunction;


pub trait Evaluateur {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error>;
}

pub fn function<T: Evaluateur + 'static>(body: T) -> Function {
    Function(Rc::new(move |args: HashMap<String, Var>,  vm: Vm| -> Result<Value, Error> {
        let mut vm = vm.clone();
        for i in args.iter() {
            vm.set_ident(Ident(i.0.clone()), i.1.clone());
        }
        body.eval(&mut vm)
        
    }))
}

#[derive(Debug, Clone)]
pub struct Vm(
    std::collections::HashMap<Ident, Var>
);

impl Vm {
    pub fn new() -> Self {
        let mut vm = Vm(HashMap::new());
        vm.use_builtin_function();
        vm
    }

    pub fn use_builtin_function(&mut self) {
        let map = BuiltinFunction::build();
        for i in map.iter() {
            self.set_ident(Ident(i.0.clone()), Var {
                value: Value::Function { name: i.0.clone(), func: Function(i.1.0.clone()), args: i.1.1.clone()},
                type_: Type::Func,
                mutable: false,
            });
        }
    }        

    pub fn set_ident(&mut self, ident: Ident, value: Var) {
        self.0.insert(ident.clone(), value);
    }

    pub fn get_ident(&self, ident: Ident) -> Option<&Var> {
        
        self.0.get(&ident)

    }

    pub fn iadd(&mut self, a: String, b: Value) -> Result<Value, Error> {
        match b {
            Value::Number(b) => {
                if self.exists(Ident(a.clone())) {
                    let v = self.get_ident(Ident(a.clone())).unwrap().clone();
                    if ! v.mutable {
                        return Err(Error::ItsAConstant(ItsAConstantError {
                            var_name: a
                        }))
                    }

                    let r = match v.value {
                        Value::Number(n) => {
                            self.set_ident(Ident(a), Var{value: Value::Number(n + b), type_: v.clone().type_, mutable: v.clone().mutable});
                            Ok(Value::None)
                        },
                        _ => Err(Error::TypeMismatch(TypeMismatchError {
                            expected: Type::Int,
                            found: v.value.get_type(),
                        })),
                    }?; 
                    if r.get_type() != v.clone().type_ {
                        return Err(Error::TypeMismatch(TypeMismatchError {
                            expected: v.type_,
                            found: r.get_type()
                        }))
                    } else {
                        Ok(r)
                    }
                } else {
                    return Err(Error::VarNotFound(VarNotFoundError {
                        var_name: a,
                    }));
                }
            },
            _ => Err(Error::TypeMismatch(TypeMismatchError {
                expected: Type::Int,
                found: Type::None,
            })),
        }
    }

    pub fn isub(&mut self, a: String, b: Value) -> Result<Value, Error> {
        match b {
            Value::Number(b) => {
                if self.exists(Ident(a.clone())) {
                    let v = self.get_ident(Ident(a.clone())).unwrap().clone();
                    if ! v.mutable {
                        return Err(Error::ItsAConstant(ItsAConstantError {
                            var_name: a
                        }))
                    }
                    let r = match v.value {
                        Value::Number(n) => {
                            self.set_ident(Ident(a), Var{value: Value::Number(n - b), type_: v.clone().type_, mutable: v.clone().mutable});
                            Ok(Value::None)
                        },
                        _ => Err(Error::TypeMismatch(TypeMismatchError {
                            expected: Type::Int,
                            found: v.value.get_type(),
                        })),
                    }?; 
                    if r.get_type() != v.clone().type_ {
                        return Err(Error::TypeMismatch(TypeMismatchError {
                            expected: v.type_,
                            found: r.get_type()
                        }))
                    } else {
                        Ok(r)
                    }
                } else {
                    return Err(Error::VarNotFound(VarNotFoundError {
                        var_name: a,
                    }));
                }
            },
            _ => Err(Error::TypeMismatch(TypeMismatchError {
                expected: Type::Int,
                found: Type::None,
            })),
        }
    }

    pub fn imul(&mut self, a: String, b: Value) -> Result<Value, Error> {
        match b {
            Value::Number(b) => {
                if self.exists(Ident(a.clone())) {
                    let v = self.get_ident(Ident(a.clone())).unwrap().clone();
                    if ! v.mutable {
                        return Err(Error::ItsAConstant(ItsAConstantError {
                            var_name: a
                        }))
                    }
                    let r = match v.value {
                        Value::Number(n) => {
                            self.set_ident(Ident(a), Var{value: Value::Number(n * b), type_: v.clone().type_, mutable: v.clone().mutable});
                            Ok(Value::None)
                        },
                        _ => Err(Error::TypeMismatch(TypeMismatchError {
                            expected: Type::Int,
                            found: v.value.get_type(),
                        })),
                    }?; 
                    if r.get_type() != v.clone().type_ {
                        return Err(Error::TypeMismatch(TypeMismatchError {
                            expected: v.type_,
                            found: r.get_type()
                        }))
                    } else {
                        Ok(r)
                    }
                } else {
                    return Err(Error::VarNotFound(VarNotFoundError {
                        var_name: a,
                    }));
                }
            },
            _ => Err(Error::TypeMismatch(TypeMismatchError {
                expected: Type::Int,
                found: Type::None,
            })),
        }
    }

    pub fn idiv(&mut self, a: String, b: Value) -> Result<Value, Error> {
        match b {
            Value::Number(b) => {
                if self.exists(Ident(a.clone())) {
                    let v = self.get_ident(Ident(a.clone())).unwrap().clone();
                    if ! v.mutable {
                        return Err(Error::ItsAConstant(ItsAConstantError {
                            var_name: a
                        }))
                    }
                    let r = match v.value {
                        Value::Number(n) => {
                            self.set_ident(Ident(a), Var{value: Value::Number(n / b), type_: v.clone().type_, mutable: v.clone().mutable});
                            Ok(Value::None)
                        },
                        _ => Err(Error::TypeMismatch(TypeMismatchError {
                            expected: Type::Int,
                            found: v.value.get_type(),
                        })),
                    }?; 
                    if r.get_type() != v.clone().type_ {
                        return Err(Error::TypeMismatch(TypeMismatchError {
                            expected: v.type_,
                            found: r.get_type()
                        }))
                    } else {
                        Ok(r)
                    }
                } else {
                    return Err(Error::VarNotFound(VarNotFoundError {
                        var_name: a,
                    }));
                }
            },
            _ => Err(Error::TypeMismatch(TypeMismatchError {
                expected: Type::Int,
                found: Type::None,
            })),
        }
    }



    pub fn exists(&self, ident: Ident) -> bool {
        self.0.contains_key(&ident)
    }





}