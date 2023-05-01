use std::rc::Rc;
use crate::value::get::{Getter, Setter};
use crate::value::{Implementation, Var};
use super::class::Class;
use super::{Object, Type, RustValue};
use crate::ast::expr::{Expr, ExprType};
use crate::interpreter::Interpreter;

#[derive(Debug, Clone, PartialEq)]
pub struct Instance {
    pub class: Class,
    pub name: String,
}

impl Instance {
    pub fn new(class: Class, name: String) -> Self {
        Instance { class, name }
    }
    pub fn create_instance(class: Class, name: String) -> Object {
        let ins = Instance::new(class.clone(), name.clone());
        Object {
            type_: Type::Instance(name.clone()),
            implementations: vec![
                Implementation::Get(Rc::new(ins.clone())),
                Implementation::Set(Rc::new(ins.clone())),
            ],
            value: RustValue::Instance(ins)
        }
    }
}

impl Getter for Instance {
    fn fetch(&self, _interpreteur: &mut Interpreter, _obj: Object,   name: Expr) -> Option<Object> {
        match *name.expr_type {
            ExprType::Ident { ident } => {
                self.class.methods.fetch(ident.lexeme).map(|v| v.value.clone())
            },
            _ => None
        }
    }
}

impl Setter for Instance {
    fn fetch(&self, name: String) -> Option<Object> {
        self.class.methods.fetch(name).map(|v| v.value.clone())
    }

    fn modif(&mut self, key: String, value: Object) -> Option<Object>  {
        self.class.methods.modify(key, Var {
            value,
            mutable: true,
            type_: Type::Any
        }).map(|v| v.value.clone())
    }
}
