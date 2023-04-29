use std::hash::Hash;
use crate::interpreter::environement::Environment;
use crate::value::{Implementation, Object, Var, Type, RustValue};
use crate::value::get;
use crate::ast::expr::{Expr, ExprType};
use crate::errors::{Error, ErrorType};
use std::rc::Rc;
use crate::{error, get_impl_if_exist};
use crate::interpreter::Interpreter;

#[derive(Clone, Debug, PartialEq)]
pub struct Namespace {
    value: Environment<String, Var>
}

impl Namespace {
    pub fn new(env: Environment<String, Var>) -> Self {
        Self {
            value: env
        }
    }

    pub fn create(self) -> Object {
        Object {
            type_: Type::Namespace,
            implementations: vec![
                Implementation::NsGet(Rc::new(self.clone())),
            ],
            value: RustValue::Namespace(self.clone())
        }
    }
}

impl get::NsGetter for Namespace {
    fn fetch(&self, interpreteur: &mut Interpreter, name: Expr) -> Option<Object> {

        match *name.expr_type {
            ExprType::Ident { ident } => {
                match self.value.fetch(ident.lexeme) {
                    Some(var) => Some(var.value),
                    None => None
                }
            },
            ExprType::Call { ref name, args: old_args }  => {
                let mut args = vec![];
                for arg in old_args {
                    args.push(arg.accept(interpreteur));
                }
                match self.fetch(&mut interpreteur.clone(), name.clone()) {
                    Some(obj) => {
                        match get_impl_if_exist!(Call, obj) {
                            Some(call) => Some(call.call(interpreteur, args, name.file.as_str())),
                            None => {
                                error!(ErrorType::TypeError, "Expected a function", 0..0, "".to_string());
                                unreachable!()
                            }
                        }
                    },
                    None => None
                }
            },
            _ => None
        }
    }
}
