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
}

impl Function {
    pub fn new(declaration: Stmt) -> Self {
        Function { declaration }
    }

    pub fn create_function(declaration: Stmt) -> Object {
        Object {
            type_: Type::Function,
            implementations: vec![
                Implementation::Call(Rc::new(Function::new(declaration)))
            ],
            value: RustValue::Function
        }
    }
}

impl Callable for Function {
    fn call(&self, _interpreter: &mut Interpreter, args: Vec<Object>, _file: &str) -> Object {
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
                            Implementation::Call(Rc::new(Function::new(self.declaration.clone())))
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
                                    Implementation::Call(Rc::new(Function::new(self.declaration.clone())))
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


