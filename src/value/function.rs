use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use super::{Object, Type};
use crate::ast::stmt::{Stmt, StmtType};
use crate::ast::expr::ExprType;

use crate::errors::{error, Error, ErrorType};
use crate::interpreter::environement::Environment;
use crate::interpreter::Interpreter;
use crate::value::callable::Callable;
use crate::value::{Implementation, RustValue, Var};


#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub declaration: Stmt,
    pub name: String
}

#[derive(Clone)]
pub struct BuiltinFunction{
    pub func: Rc<dyn Fn(&mut Interpreter, &mut Object, &mut Vec<Object>, &str) -> Object>,
    pub id: i32
}

impl Debug for BuiltinFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("<builtin-function>").fmt(f)
    }
}

impl PartialEq for BuiltinFunction {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl BuiltinFunction {
    pub fn new(func: Rc<dyn Fn(&mut Interpreter, &mut Object, &mut Vec<Object>, &str) -> Object>, id: i32) -> Self {
        Self { func, id }
    }

    pub fn create_object(&self) -> Object {
        Object {
            type_: Type::Function,
            implementations: vec![
                Implementation::Call(Rc::new(self.clone()))
            ],
            value: RustValue::Function
        }
    }
}

impl Callable for BuiltinFunction {
    fn call(&self, interpreter: &mut Interpreter, args: &mut Vec<Object>, file: &str) -> Object {
        panic!("cant call on builtin stdlib function")
    }

    fn method(&self, interpreteur: &mut Interpreter, this: &mut Object, args: &mut Vec<Object>, file: &str) -> Object {
        (self.func)(interpreteur, this, args, file)
    }
}

impl Function {
    pub fn new(declaration: Stmt, name: Option<String>) -> Self {
        let name = if let Some(e) = name {
            e
        } else {
            "<function>".to_string()
        };

        Function { declaration, name }
    }

    pub fn create_function(declaration: Stmt) -> Object {
        let name = if let StmtType::Function { name, .. } = *declaration.stmt_type.clone() { name } else { panic!("not a function") };
        Object {
            type_: Type::Function,
            implementations: vec![
                Implementation::Call(Rc::new(Function::new(declaration, Some(name.lexeme))))
            ],
            value: RustValue::Function
        }
    }
}

macro_rules! function_call_or_this {
    (call) => {
        fn call(&self, _interpreter: &mut Interpreter, args: &mut Vec<Object>, _file: &str) -> Object {
            let mut env = Environment::new(None);
            let mut new_interpreter = Interpreter::new();
            env = new_interpreter.env.clone();

            let mut i = 0;
            match &*self.declaration.stmt_type {
                StmtType::Function { args: params, name, body } => {
                    env.define(name.lexeme.to_string(), Var {
                        value: Object {
                            type_: Type::Function,
                            implementations: vec![
                                Implementation::Call(Rc::new(Function::new(self.declaration.clone(), Some(name.lexeme.clone()))))
                            ],
                            value: RustValue::Function
                        },
                        mutable: false,
                        type_: Type::Function
                    });
                    for arg in params {
                        env.define(arg.clone(), Var {
                            value: args[i].clone(),
                            mutable: false,

                            type_: args[i].type_.clone()
                        });
                        i += 1;
                    }
                    new_interpreter.env = env;
                    body.clone().accept(&mut new_interpreter)

                },
                StmtType::Expression { expr } => {
                    if let ExprType::Lambda { args: params, body } = &*expr.expr_type {

                        for arg in params {
                            env.define(arg.lexeme.clone(), Var {
                                value: args[i].clone(),
                                mutable: false,
                                type_: args[i].type_.clone()
                            });
                            env.define("self".to_string(), Var {
                                value: Object {
                                    type_: Type::Function,
                                    implementations: vec![
                                        Implementation::Call(Rc::new(Function::new(self.declaration.clone(), None)))
                                    ],
                                    value: RustValue::Function
                                },
                                mutable: false,
                                type_: Type::Function
                            }); // for recursive calls to the function, in a lambda , we need to define self, so that we can call the function recursively
                            i += 1;
                        }
                        new_interpreter.env = env;
                        body.clone().accept(&mut new_interpreter)
                    } else {
                        error!(ErrorType::TypeError, "Expected a function", 0..0, "".to_string());
                        unreachable!()
                    }
                },
                _ => {
                    error!(ErrorType::TypeError, "Expected a function", 0..0, "".to_string());
                    unreachable!()
                }
            }
        }
    };

    (this) => {
        fn method(&self, _interpreter: &mut Interpreter,this: &mut Object,  args: &mut Vec<Object>, _file: &str) -> Object {
            let mut env = Environment::new(None);
            let mut new_interpreter = Interpreter::new();
            env = new_interpreter.env.clone();

            let mut i = 0;
            match &*self.declaration.stmt_type {
                StmtType::Function { args: params, name, body } => {
                    env.define(name.lexeme.to_string(), Var {
                        value: Object {
                            type_: Type::Function,
                            implementations: vec![
                                Implementation::Call(Rc::new(Function::new(self.declaration.clone(), Some(name.lexeme.clone()))))
                            ],
                            value: RustValue::Function
                        },
                        mutable: false,
                        type_: Type::Function
                    });
                    for arg in params {
                        env.define(arg.clone(), Var {
                            value: args[i].clone(),
                            mutable: false,

                            type_: args[i].type_.clone()
                        });
                        i += 1;
                    }

                    env.define("this".to_string(), Var {
                        value: this.clone(),
                        type_: this.type_.clone(),
                        mutable: false
                    });
                    new_interpreter.env = env;
                    body.clone().accept(&mut new_interpreter)

                },
                StmtType::Expression { expr } => {
                    if let ExprType::Lambda { args: params, body } = &*expr.expr_type {

                        for arg in params {
                            env.define(arg.lexeme.clone(), Var {
                                value: args[i].clone(),
                                mutable: false,
                                type_: args[i].type_.clone()
                            });
                            env.define("self".to_string(), Var {
                                value: Object {
                                    type_: Type::Function,
                                    implementations: vec![
                                        Implementation::Call(Rc::new(Function::new(self.declaration.clone(), None)))
                                    ],
                                    value: RustValue::Function
                                },
                                mutable: false,
                                type_: Type::Function
                            }); // for recursive calls to the function, in a lambda , we need to define self, so that we can call the function recursively
                            i += 1;
                        }
                        new_interpreter.env = env;
                        body.clone().accept(&mut new_interpreter)
                    } else {
                        error!(ErrorType::TypeError, "Expected a function", 0..0, "".to_string());
                        unreachable!()
                    }
                },
                _ => {
                    error!(ErrorType::TypeError, "Expected a function", 0..0, "".to_string());
                    unreachable!()
                }
            }
        }
    }
}

impl Callable for Function {
    function_call_or_this!(call);
    function_call_or_this!(this);
}




