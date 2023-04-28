use std::rc::Rc;

use super::{Object, Type};
use crate::ast::stmt::{Stmt, StmtType};
use crate::ast::expr::{Expr};
use crate::errors::{error, Error, ErrorType};
use crate::interpreter::environement::Environment;
use crate::interpreter::Interpreter;
use crate::value::callable::Callable;
use crate::value::{Implementation, RustValue, Var};
use crate::value::litteral::none;


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
    fn call(&self, _interpreter: &mut Interpreter, args: Vec<Object>) -> Object {
        let mut env = Environment::new(None);
        let mut new_interpreteur = Interpreter::new_with_env(env.clone());
        let mut i = 0;
        match &*self.declaration.stmt_type {
            StmtType::Function { args: params, name, body } => {
                for arg in params {
                    env.define(arg.clone(), Var {
                        value: args[i].clone(),
                        mutable: false,

                        type_: args[i].type_.clone()
                    });
                    i += 1;
                }
                new_interpreteur.env = env;
                body.clone().accept(&mut new_interpreteur)
            },
            _ => {
                error!(ErrorType::TypeError, "Expected a function", 0..0, "".to_string());
                unreachable!()
            }
        }
    }
}


