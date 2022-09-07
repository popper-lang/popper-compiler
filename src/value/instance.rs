use super::class::Class;
use super::{Object, Type};

#[derive(Debug, Clone, PartialEq)]
pub struct Instance {
    pub class: Class,
    pub name: String
}


impl Instance {
    pub fn new(class: Class, name: String) -> Self {
        Instance {
            class,
            name
        }
    }
}

impl Object for Instance {
    fn display_value(&self) -> String {
        format!("instance of {}", self.class.name)
    }

    fn get_type(&self) -> Type {
        Type::Instance(self.class.name.clone())
    }
}