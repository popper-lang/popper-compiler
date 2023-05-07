use crate::interpreter::environement::Environment;
use crate::value::{Object, Value, Type, Var};



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

    pub fn create_class(name: &str) -> Object {
        Object {
            type_: Type::Class(name.to_string()),
            implementations: vec![],
            value: Value::Class(Class::new(name.to_string())),
            tags: std::default::Default::default()

        }
    }
}
