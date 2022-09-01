use super::callable::Callable;
use crate::ast::stmt::Stmt;
use crate::interpreter::Interpreter;
use crate::interpreter::environement::Environment;
use crate::lexer::{Token, TokenType};
use super::Value;
use crate::errors::error;

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub declaration: Stmt
}

impl Callable for Function {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Value>) -> Value {
        let mut env = Environment::new(Some(interpreter.env.clone()));
        let old_interpreter = interpreter.clone();
        let fun = match &self.declaration {
            Stmt::Function {args, body, .. } => (args, body),
            _ => error!("must be a function declaration")

        };

        if fun.0.len() != args.len() {
            error!("the number of arguments doesn't match with the number of arguments of the function")
        } else {
            for i in 0..fun.0.len() {
                env.define(fun.0[i].clone(), super::Var { value: args[i].clone(), mutable: false, type_: args[i].get_type() });
            }
            let res = fun.1.clone().accept(&mut Interpreter::new_with_env(env));
            interpreter.env = old_interpreter.env;
            res.clone()
        }




    }
}