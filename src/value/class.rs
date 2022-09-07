use crate::interpreter::{environement::Environment};
use crate::value::{Var, Object, Type};

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

impl Object for Class {
    fn display_value(&self) -> String {
        format!("class '{}'", self.name)
    }

    fn get_type(&self) -> Type {
        Type::Class(self.name.clone())
    }
}

