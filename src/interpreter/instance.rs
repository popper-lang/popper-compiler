use super::class::Class;


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