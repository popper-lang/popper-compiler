use std::rc::Rc;

use crate::ast::stmt::Stmt;
use crate::interpreter::Interpreter;
use crate::interpreter::environement::Environment;
use super::{Object, Type};
use crate::errors::error;

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub declaration: Stmt
}





impl Object for Function {
    fn display_value(&self) -> String {
        "Function".to_string()
    }

    fn get_type(&self) -> Type {
        Type::Func
    }

    fn call(&self, interpreter: &mut Interpreter, args: Vec<Rc<dyn Object>>) -> Rc<dyn Object> {
        let mut env = Environment::new(Some(interpreter.env.clone()));
        let old_interpreter = interpreter.clone();
        let _args;
        let _body;
        match &self.declaration {
            Stmt::Function {args: a, body: b, .. } => {
                _args = a;
                _body = b.clone();
            },
            _ => error!("must be a function declaration")

        };

        if _args.len() != args.len() {
            error!("the number of arguments doesn't match with the number of arguments of the function")
        } else {
            for i in 0.._args.len() {
                env.define(_args[i].clone(), super::Var { value: args[i].clone(), mutable: false, type_: args[i].get_type() });
            }
            let res = _body.accept(&mut Interpreter::new_with_env(env));
            interpreter.env = old_interpreter.env;
            res
        }
    }


}