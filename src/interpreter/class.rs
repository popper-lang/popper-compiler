use super::instance::Instance;
use super::{environement::Environment, Interpreter};
use crate::value::callable::Callable;
use crate::value::{Value, Var};

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    pub name: String,
    pub methods: Environment<String, Var>,
}

impl Class {
    pub fn new(name: String) -> Self {
        Class {
            name,
            methods: Environment::new(None),
        }
    }

}

impl Callable for Class {
    fn call(&self, _interpreter: &mut Interpreter, _args: Vec<Value>) -> Value {
        Value::Instance(Instance::new(self.clone(), "".to_string()))
    }
}